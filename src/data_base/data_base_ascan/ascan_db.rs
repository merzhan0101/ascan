//! Работа с базами Асканов.
//!
//! Главное правило: каждая линия (КПЦ линия №1, КПЦ линия №2, Пролёт №6)
//! опрашивается отдельно, параллельно и со своим таймаутом. Если с одной
//! линией нет связи, остальные всё равно отдают данные, а для недоступной
//! линии возвращается текст ошибки.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::future::Future;
use std::path::Path;
use std::time::Duration;

use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use dotenv::dotenv;
use futures_util::TryStreamExt;
use tiberius::{Client, Query, QueryItem, Row};
use tokio::net::TcpStream;
use tokio_util::compat::Compat;

use crate::data_base::client_db::{client_6pr, client_t};
use crate::structures::ascan_struct::{DataAscan, LineResult, WheelLineData, WheelPlace};
use crate::utils::utils_metod as utils;

type DbClient = Client<Compat<TcpStream>>;

// ---------------------------------------------------------------------------
// Линии
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Line {
    One,
    Two,
    Pr6,
}

impl Line {
    /// Ключ линии в JSON-ответе.
    pub fn key(self) -> &'static str {
        match self {
            Line::One => "line_one",
            Line::Two => "line_two",
            Line::Pr6 => "pr_6",
        }
    }

    /// Название для сообщений пользователю.
    pub fn title(self) -> &'static str {
        match self {
            Line::One => "КПЦ, линия №1",
            Line::Two => "КПЦ, линия №2",
            Line::Pr6 => "Пролёт №6",
        }
    }

    fn table_var(self) -> &'static str {
        match self {
            Line::One => "NAME_TABLE_ONE_LINE",
            Line::Two => "NAME_TABLE_TWO_LINE",
            Line::Pr6 => "NAME_TABLE_6PR",
        }
    }

    fn img_dir_var(self) -> &'static str {
        match self {
            Line::One => "ASCAN_ONE_LINE_DIR",
            Line::Two => "ASCAN_TWO_LINE_DIR",
            Line::Pr6 => "ASCAN_6PR_DIR",
        }
    }

    /// Колонка с номером партии.
    fn batch_col(self) -> &'static str {
        match self {
            Line::Pr6 => "Batch_number",
            _ => "DocNumber",
        }
    }

    /// Колонка с номером плавки.
    fn malting_col(self) -> &'static str {
        match self {
            Line::Pr6 => "Treatment_number",
            _ => "BatchNumber",
        }
    }

    async fn connect(self) -> Result<DbClient> {
        match self {
            Line::Pr6 => client_6pr().await,
            _ => client_t().await,
        }
    }

    fn table(self) -> Result<String> {
        dotenv().ok();
        env::var(self.table_var()).map_err(|_| anyhow!("не задана переменная {}", self.table_var()))
    }
}

// ---------------------------------------------------------------------------
// Таймауты
// ---------------------------------------------------------------------------

fn secs_from_env(name: &str, default: u64) -> Duration {
    dotenv().ok();
    let secs = env::var(name).ok().and_then(|v| v.trim().parse().ok()).unwrap_or(default);
    Duration::from_secs(secs)
}

/// Сколько ждать ответа от базы одной линии (подключение + запрос).
fn db_timeout() -> Duration {
    secs_from_env("DB_TIMEOUT_SECS", 10)
}

/// Сколько ждать поиска картинок в папке линии (папка может быть сетевой).
fn img_timeout() -> Duration {
    secs_from_env("IMG_TIMEOUT_SECS", 10)
}

/// Выполняет `fut`, но не дольше `dur`. Зависание превращается в обычную ошибку.
async fn limited<T>(dur: Duration, fut: impl Future<Output = Result<T>>) -> Result<T> {
    match tokio::time::timeout(dur, fut).await {
        Ok(res) => res,
        Err(_) => Err(anyhow!("нет ответа за {} с", dur.as_secs())),
    }
}

fn no_connection(line: Line, err: &anyhow::Error) -> String {
    eprintln!("[ascan] {}: {:#}", line.title(), err);
    format!("{}: нет связи ({:#})", line.title(), err)
}

// ---------------------------------------------------------------------------
// Чтение ячеек без паник
// ---------------------------------------------------------------------------

/// Читает ячейку как строку независимо от её типа в базе (строка, число, дата).
/// NULL и неизвестные типы дают None — строка с плохой ячейкой не роняет запрос.
fn cell_str(row: &Row, idx: usize) -> Option<String> {
    if let Ok(v) = row.try_get::<&str, _>(idx) {
        return v.map(|s| s.trim().to_string());
    }
    if let Ok(v) = row.try_get::<i64, _>(idx) {
        return v.map(|n| n.to_string());
    }
    if let Ok(v) = row.try_get::<i32, _>(idx) {
        return v.map(|n| n.to_string());
    }
    if let Ok(v) = row.try_get::<i16, _>(idx) {
        return v.map(|n| n.to_string());
    }
    if let Ok(v) = row.try_get::<u8, _>(idx) {
        return v.map(|n| n.to_string());
    }
    if let Ok(v) = row.try_get::<f64, _>(idx) {
        return v.map(|n| n.to_string());
    }
    if let Ok(v) = row.try_get::<f32, _>(idx) {
        return v.map(|n| n.to_string());
    }
    if let Ok(v) = row.try_get::<NaiveDateTime, _>(idx) {
        return v.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string());
    }
    None
}

fn file_name_of(path: &str) -> Option<String> {
    Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
}

// ---------------------------------------------------------------------------
// Списки партий и плавок (GET /ascan)
// ---------------------------------------------------------------------------

async fn distinct(client: &mut DbClient, table: &str, col: &str) -> Result<Vec<String>> {
    let mut stream = Query::new(format!("select distinct {} from {}", col, table))
        .query(client)
        .await?;
    let mut out = Vec::new();
    while let Some(item) = stream.try_next().await? {
        if let QueryItem::Row(r) = item {
            if let Some(v) = cell_str(&r, 0) {
                if !v.is_empty() {
                    out.push(v);
                }
            }
        }
    }
    Ok(out)
}

/// Партии и плавки одной линии.
async fn lists_for(line: Line) -> Result<(Vec<String>, Vec<String>)> {
    let table = line.table()?;
    let mut client = line.connect().await?;
    let batches = distinct(&mut client, &table, line.batch_col()).await?;
    let maltings = distinct(&mut client, &table, line.malting_col()).await?;
    Ok((batches, maltings))
}

/// Собирает списки со всех линий. Недоступные линии пропускаются и попадают в `errors`.
pub async fn source_lists() -> DataAscan {
    let t = db_timeout();
    let (one, two, pr6) = tokio::join!(
        limited(t, lists_for(Line::One)),
        limited(t, lists_for(Line::Two)),
        limited(t, lists_for(Line::Pr6)),
    );

    let mut errors = BTreeMap::new();

    let mut kpc_batches = BTreeSet::new();
    let mut kpc_maltings = BTreeSet::new();
    for (line, res) in [(Line::One, one), (Line::Two, two)] {
        match res {
            Ok((b, m)) => {
                kpc_batches.extend(b);
                kpc_maltings.extend(m);
            }
            Err(e) => {
                errors.insert(line.key().to_string(), no_connection(line, &e));
            }
        }
    }

    let (pr6_batches, pr6_maltings) = match pr6 {
        Ok((b, m)) => (
            b.into_iter().collect::<BTreeSet<_>>(),
            m.into_iter().collect::<BTreeSet<_>>(),
        ),
        Err(e) => {
            errors.insert(Line::Pr6.key().to_string(), no_connection(Line::Pr6, &e));
            (BTreeSet::new(), BTreeSet::new())
        }
    };

    DataAscan {
        batch_number_kpc: kpc_batches.into_iter().collect(),
        batch_number_6pr: pr6_batches.into_iter().collect(),
        malting_for_kpc: kpc_maltings.into_iter().collect(),
        malting_for_6pr: pr6_maltings.into_iter().collect(),
        errors,
    }
}

// ---------------------------------------------------------------------------
// Колёса по партии / плавке (POST /api/wheel)
// ---------------------------------------------------------------------------

/// Одна строка из базы до поиска картинки.
struct RawWheel {
    hot_number: Option<String>,
    malting: Option<String>,
    batch: Option<String>,
    task: Option<String>,
    date_time: Option<String>,
    file_name: Option<String>,
}

async fn fetch_rows(line: Line, by_malting: bool, pattern: &str) -> Result<Vec<RawWheel>> {
    let table = line.table()?;
    let col = if by_malting { line.malting_col() } else { line.batch_col() };

    let sql = match line {
        // WheelHotNumber — горячая маркировка, BatchNumber — плавка, TaskNumber — задание,
        // DATACODE — путь к файлу, DocNumber — партия
        Line::One | Line::Two => format!(
            "select WheelHotNumber, BatchNumber, TaskNumber, DATACODE, DATE_AND_TIME, DocNumber \
             from {} where {} like @P1",
            table, col
        ),
        Line::Pr6 => format!(
            "select Barcode, Treatment_number, Path, Batch_number from {} where {} like @P1",
            table, col
        ),
    };

    let mut client = line.connect().await?;
    let mut query = Query::new(sql);
    query.bind(pattern);
    let mut stream = query.query(&mut client).await?;

    let mut rows = Vec::new();
    while let Some(item) = stream.try_next().await? {
        let QueryItem::Row(r) = item else { continue };
        let wheel = match line {
            Line::One | Line::Two => RawWheel {
                hot_number: cell_str(&r, 0),
                malting: cell_str(&r, 1),
                task: cell_str(&r, 2),
                file_name: cell_str(&r, 3).and_then(|p| file_name_of(&p)),
                date_time: cell_str(&r, 4),
                batch: cell_str(&r, 5),
            },
            Line::Pr6 => {
                let path = cell_str(&r, 2);
                RawWheel {
                    hot_number: cell_str(&r, 0),
                    malting: cell_str(&r, 1),
                    task: None,
                    file_name: path.as_deref().and_then(|p| file_name_of(&format!("{}.png", p))),
                    date_time: path,
                    batch: cell_str(&r, 3),
                }
            }
        };
        rows.push(wheel);
    }
    Ok(rows)
}

/// Ищет картинки в папке линии. Папка может быть сетевой и «висеть»,
/// поэтому поиск идёт в отдельном потоке и с таймаутом.
async fn find_images(line: Line, names: Vec<Option<String>>) -> Result<Vec<Option<String>>> {
    dotenv().ok();
    let dir = env::var(line.img_dir_var())
        .map_err(|_| anyhow!("не задана переменная {}", line.img_dir_var()))?;
    let lookup = tokio::task::spawn_blocking(move || {
        names
            .iter()
            .map(|n| n.as_deref().and_then(|n| utils::find_file_by_date_path(&dir, n)))
            .collect::<Vec<_>>()
    });
    limited(img_timeout(), async { lookup.await.map_err(|e| anyhow!(e)) }).await
}

async fn line_result(line: Line, by_malting: bool, pattern: &str) -> LineResult {
    let rows = match limited(db_timeout(), fetch_rows(line, by_malting, pattern)).await {
        Ok(rows) => rows,
        Err(e) => {
            return LineResult { data: None, error: Some(no_connection(line, &e)) };
        }
    };

    let names = rows.iter().map(|r| r.file_name.clone()).collect();
    let (paths, error) = match find_images(line, names).await {
        Ok(p) => (p, None),
        Err(e) => {
            eprintln!("[ascan] {}: картинки недоступны: {:#}", line.title(), e);
            (
                vec![None; rows.len()],
                Some(format!("{}: картинки недоступны ({:#})", line.title(), e)),
            )
        }
    };

    let mut data = WheelLineData::default();
    for (w, img) in rows.into_iter().zip(paths) {
        data.hot_number.push(w.hot_number);
        data.malting_namber.push(w.malting);
        data.batch_number.push(w.batch);
        data.task_number.push(w.task);
        data.date_time.push(w.date_time);
        data.path_img.push(img);
    }

    LineResult { data: Some(data), error }
}

/// Колёса по партии или плавке. Линии КПЦ опрашиваются параллельно и независимо.
pub async fn wheels(place: &str, by_malting: bool, value: &str) -> WheelPlace {
    let pattern = format!("%{}%", value);
    if place == "КПЦ" {
        let (one, two) = tokio::join!(
            line_result(Line::One, by_malting, &pattern),
            line_result(Line::Two, by_malting, &pattern),
        );
        WheelPlace { line_one: Some(one), line_two: Some(two), pr_6: None }
    } else {
        let pr6 = line_result(Line::Pr6, by_malting, &pattern).await;
        WheelPlace { line_one: None, line_two: None, pr_6: Some(pr6) }
    }
}
