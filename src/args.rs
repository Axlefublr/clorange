use crate::APP_NAME;
use crate::DATA_DIR_ENV_VAR;
use clap::Parser;
use const_format::formatcp;
use std::path::PathBuf;

const ABOUT: &str = formatcp!("\
Clorange will first pick its data directory, where it will store the counter files.
Each counter uses its own file.
It's done this way (instead of a singular json file, for example) so you can more easily integrate the counter files into your workflows.
Maybe you have another program that depends on a counter file: it would be far more annoying to *have to* use clorange to read it.

By default, clorange makes a directory with its app name in your OS' data directory to store the counters.
You can override it by setting the environment variable {DATA_DIR_ENV_VAR} to the path you want.
An even stronger override is using the -d / --data flags.

Subdirectories in the provided paths don't have to exist already, {APP_NAME} will make them for you.");

#[derive(Parser)]
#[command(author, version, long_about = ABOUT)]
pub struct Args {
	/// Specify the directory to make the counter files in
	#[arg(short, long, value_name = "PATH")]
	pub data: Option<PathBuf>,
}
