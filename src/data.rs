use crate::APP_NAME;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

const DATA_DIR: &str = APP_NAME;

pub fn ensure_dir_created(dir: &Path) -> io::Result<()> {
	fs::create_dir_all(dir)
}

pub fn get_data_dir() -> Result<PathBuf, io::Error> {
	let cache_dir = dirs::cache_dir()
		.ok_or_else(|| io::Error::new(ErrorKind::NotFound, "Cache directory not found"))?;
	Ok(cache_dir.join(DATA_DIR))
}
