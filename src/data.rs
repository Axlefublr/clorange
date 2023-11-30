use crate::APP_NAME;
use crate::DATA_DIR_ENV_VAR;
use std::env;
use std::env::VarError;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
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
            format!("Path is invalid: {}", path.display()).as_str(),
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

pub fn read(counter: &Path) -> Result<f64, Box<dyn Error>> {
    Ok(fs::read_to_string(counter)?.trim().parse()?)
}

pub fn write(counter: &Path, contents: f64) -> Result<(), io::Error> {
    fs::write(counter, contents.to_string())
}

pub fn read_write<T>(counter: &Path, action: T) -> Result<(), Box<dyn Error>>
where
    T: FnOnce(f64) -> f64,
{
    let mut file = OpenOptions::new().read(true).write(true).open(counter)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let value: f64 = contents.trim().parse()?;
    let value = action(value);
    file.set_len(0)?;
    file.rewind()?;
    file.write_all(value.to_string().as_bytes())?;
    file.flush()?;
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
