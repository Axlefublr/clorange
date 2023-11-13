use args::Args;
use clap::Parser;
use std::error::Error;
use std::io::ErrorKind;
use std::path::PathBuf;

const APP_NAME: &str = "clorange";

mod args;
mod data;

fn main() -> Result<(), Box<dyn Error>> {
	let Args { here } = Args::parse();
	let data_dir = if here {
		PathBuf::from(".")
	} else {
		data::get_data_dir().unwrap_or_else(|error| {
			eprintln!("{}. Using current directory instead.\nUse the flag --here (or -c) to opt into this behavior on your own.", error);
			PathBuf::from(".")
		})
	};
	Ok(())
}
