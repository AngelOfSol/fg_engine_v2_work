use std::fs::File;
use std::path::PathBuf;
pub fn create_new_replay_file(folder: &str) -> std::io::Result<File> {
    let mut path = PathBuf::new();
    path.push("replay");
    if !path.exists() {
        std::fs::create_dir(&path)?;
    }
    path.push(folder);
    if !path.exists() {
        std::fs::create_dir(&path)?;
    }

    let filename = chrono::Local::now().format("%Y-%m-%d %H%M.rep").to_string();

    path.push(filename);

    File::create(path)
}
