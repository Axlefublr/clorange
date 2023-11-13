use std::error::Error;

mod data;

fn main() -> Result<(), Box<dyn Error>> {
	data::ensure_dir_created();
	Ok(())
}