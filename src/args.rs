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

Each counter file contains a single number, that you modify with subcommands.
The number is a float, so float operations are supported,
but you can just specify your argument numbers as integers (1 vs 1.0)");

const DATA_ABOUT: &str = formatcp!(
	"\
Specify the directory to make the counter files in.
Subdirectories don't have to exist already, {APP_NAME} will make them for you.
The data directory acts as a \"current working directory\" essentially,
since you can still specify paths in subdirectories for the counter file."
);

const COUNTER_ABOUT: &str = formatcp!(
	"\
Specify the filename of a counter you want to (possibly) modify.
Counter files, along with all their parent directories, are created automatically
if the subcommand you're calling isn't \"clear\".
The default value in a new counter file is 0."
);

#[derive(Parser)]
#[command(author, version, long_about = ABOUT)]
pub struct Args {
	#[arg(short, long, value_name = "PATH", help = DATA_ABOUT)]
	pub data: Option<PathBuf>,
	#[arg(help = COUNTER_ABOUT)]
	pub counter: PathBuf,
}
