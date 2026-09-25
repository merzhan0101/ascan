use std::path::Path;

/// Ищет файл по дате в имени: <root>/<ГГГГ>/<ММ>/<ДД>/<имя>.
/// Имя файла начинается с даты и времени, например 20250323120740.png.
pub fn find_file_by_date_path(root_dir: &str, filename: &str) -> Option<String> {
    // .get() вместо [..] — чтобы странное имя файла не вызывало панику
    let year = filename.get(0..4)?;
    let month = filename.get(4..6)?;
    let day = filename.get(6..8)?;
    if filename.len() < 14 {
        return None;
    }

    let full_path = Path::new(root_dir)
        .join(year)
        .join(month)
        .join(day)
        .join(filename);

    if full_path.is_file() {
        Some(full_path.to_string_lossy().into_owned())
    } else {
        None
    }
}
