use std::path::Path;
use std::{fs, io};
use serde::export::Option::Some;

pub fn ensure_file_exists(file_path: &str, init_text: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, init_text)?;
    }
    Ok(())
}
