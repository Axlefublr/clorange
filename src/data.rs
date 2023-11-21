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

pub fn ensure_exists(path: &Path) -> io::Result<()> {
    let dir = path.parent().ok_or_else(|| {
        io::Error::new(
            ErrorKind::NotFound,
            format!(
                "You didn't specify a data directory.\nSpecified path is: {}",
                path.display()
            )
            .as_str(),
        )
    })?;
    path.file_name()
        .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, ".. or . are not valid filenames"))?;
    fs::create_dir_all(dir)?;
    if !path.exists() {
        fs::write(path, DEFAULT_START_VALUE)?
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
