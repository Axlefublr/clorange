use clap::Parser;

#[derive(Parser)]
pub struct Args {
	/// Use the current working directory as the data directory.
	/// Usually, your OS' cache directory is used.
	#[arg(short = 'c', long)]
	pub here: bool
}