use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use crate::data_base::data_base_ascan::ascan_db as db;

/// Списки партий и плавок для выпадающего списка.
/// Если какая-то линия недоступна, её списки просто не попадут сюда,
/// а причина будет в `errors` (ключи: "line_one", "line_two", "pr_6").
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataAscan{
    pub batch_number_kpc:Vec<String>,
    pub batch_number_6pr:Vec<String>,
    pub malting_for_kpc:Vec<String>,
    pub malting_for_6pr:Vec<String>,
    pub errors: BTreeMap<String, String>,
}

impl DataAscan {
    pub async fn new()-> Self{
        db::source_lists().await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFilter{
    pub place: String,
    pub batch_number: Option<String>,
    pub malting: Option<String>
}

/// Колёса одной линии. Все массивы одинаковой длины: i-й элемент каждого
/// массива относится к одному и тому же колесу.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WheelLineData{
    pub hot_number: Vec<Option<String>>,
    pub malting_namber: Vec<Option<String>>,
    pub batch_number:Vec<Option<String>>,
    pub task_number:Vec<Option<String>>,
    pub path_img: Vec<Option<String>>,
    pub date_time: Vec<Option<String>>
}

/// Результат по одной линии.
/// - data есть, error нет — всё хорошо;
/// - data нет, error есть — с линией нет связи;
/// - data есть и error есть — данные получены, но есть проблема (например, картинки недоступны).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LineResult{
    pub data: Option<WheelLineData>,
    pub error: Option<String>,
}

/// Ответ /api/wheel. Линии, которые не относятся к выбранному участку, равны null.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WheelPlace{
    pub pr_6: Option<LineResult>,
    pub line_one: Option<LineResult>,
    pub line_two: Option<LineResult>
}
