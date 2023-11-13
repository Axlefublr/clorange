use crate::APP_NAME;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

const DATA_SUBDIR: &str = APP_NAME;

pub fn ensure_exists(dir: &Path) -> io::Result<()> {
	fs::create_dir_all(dir)
}

pub fn get_data_dir() -> Result<PathBuf, io::Error> {
	let data_dir = dirs::data_dir()
		.ok_or_else(|| io::Error::new(ErrorKind::NotFound, "Data directory not found"))?;
	Ok(data_dir.join(DATA_SUBDIR))
}
