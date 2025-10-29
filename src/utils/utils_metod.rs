use std::path::Path;

pub fn find_file_by_date_path(root_dir: &str, filename: &str) -> Option<String> {
    if filename.len() < 14 {
        return None;
    }

    let year = &filename[0..4];
    let month = &filename[4..6];
    let day = &filename[6..8];

    let full_path = Path::new(root_dir)
        .join(year)
        .join(month)
        .join(day)
        .join(filename);

    if full_path.exists() && full_path.is_file() {
        Some(full_path.to_string_lossy().into_owned())
    } else {
        None
    }
}