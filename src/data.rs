use crate::APP_NAME;
use crate::DATA_DIR_ENV_VAR;
use std::env;
use std::env::VarError;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

const DATA_SUBDIR: &str = APP_NAME;
const DEFAULT_START_VALUE: &str = "0.0";

pub fn ensure_dirs_exist(dir: &Path) -> io::Result<()> {
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}

pub fn ensure_file_exists(path: &Path) -> Result<(), io::Error> {
    let dir = path.parent().ok_or_else(|| {
        io::Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Path is invalid: {}",
                path.display()
            )
            .as_str(),
        )
    })?;
    path.file_name()
        .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, ".. or . are not valid filenames"))?;
    ensure_dirs_exist(dir)?;
    if !path.exists() {
        fs::write(path, DEFAULT_START_VALUE)?;
    }
    Ok(())
}

pub fn default_location() -> Result<PathBuf, io::Error> {
    let data_dir = dirs::data_dir().ok_or_else(|| {
        io::Error::new(
            ErrorKind::NotFound,
            "Failed to retrieve path of the OS' data directory",
        )
    })?;
    Ok(data_dir.join(DATA_SUBDIR))
}

pub fn get_env_location() -> Result<PathBuf, VarError> {
    Ok(env::var(DATA_DIR_ENV_VAR)?.into())
}
