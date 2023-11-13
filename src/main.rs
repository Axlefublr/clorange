use args::Args;
use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

const APP_NAME: &str = "clorange";

mod args;
mod data;

fn main() -> Result<(), Box<dyn Error>> {
	let Args { data } = Args::parse();
	let data = match data {
		Some(path) => path,
		None => data::get_data_dir().map_err(|error| {
			format!("{}. Specify your own with --data / -d.", error)
		})?
	};
	data::ensure_exists(&data)?;
	Ok(())
}
