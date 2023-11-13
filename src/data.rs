use std::fs;

const DATA_DIR: &str = "clorange";

pub fn ensure_dir_created() {
	let data_dir = dirs::cache_dir()
		.expect("cache directory should exist")
		.join(DATA_DIR);
	fs::create_dir_all(data_dir).expect("should've created application specific data directory");
}
