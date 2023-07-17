use std::process::ExitCode;

fn main() -> ExitCode {
	if let Err(msg) = ensure_dir_created() {
		eprintln!("{msg}");
		return ExitCode::FAILURE;
	};
	ExitCode::SUCCESS
}

fn ensure_dir_created() -> Result<(), &'static str> {
	let home = home::home_dir();
	if home.is_none() {
		return Err("couldn't find the HOME directory");
	}
	Ok(())
}
