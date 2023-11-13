use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
	/// Specify the directory to make the counter files in.
	///
	/// Subdirectories don't have to exist already, clorange will make them for you.
	///
	/// By default, clorange makes a directory with its app name in your OS' data directory.
	#[arg(short, long)]
	pub data: Option<PathBuf>
}