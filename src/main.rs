use args::Args;
use clap::Parser;
use std::env::VarError;
use std::error::Error;

const APP_NAME: &str = "clorange";
const DATA_DIR_ENV_VAR: &str = "CLORANGE_DATA_DIR";

mod args;
mod data;

fn main() -> Result<(), Box<dyn Error>> {
	let Args {
		data,
		counter,
		action,
	} = Args::parse();
	// I'd love to use more .unwrap_or_else()s here, but ?
	let data = match data {
		Some(path) => path,
		None => {
			match data::get_env_location() {
					Ok(path) => path,
					Err(error) => match error {
						VarError::NotUnicode(value) => {
							Err(format!("You specified the {0} environment variable, but it contains invalid unicode.\nValue of {0} specified in backticks: `{1:?}`", DATA_DIR_ENV_VAR, value))?
						},
						VarError::NotPresent => data::default_location().map_err(|error| {
							format!("{}. Specify your own with --data / -d.", error)
						})?,
					},
			}
		}
	};
	let counter = data.join(counter);
	data::ensure_exists(&counter)?;
	Ok(())
}
