use directories::ProjectDirs;
use std::{fs, path::PathBuf};

pub fn get_file_path() -> Option<PathBuf> {
    if let Some(dirs) = ProjectDirs::from("", "fvt inc.", "toldo") {
        let proj_data_dir = dirs.data_dir();
        if let Err(e) = fs::create_dir_all(proj_data_dir) {
            eprintln!("Failed creating data directory: {}", e);
            return None;
        }

        let mut proj_data_path: PathBuf = proj_data_dir.to_path_buf();
        proj_data_path.push("tasks.json");
        Some(proj_data_path)
    } else {
        None
    }
}
