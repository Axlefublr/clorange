#![deny(unused_must_use)]

use args::Action;
use args::Args;
use clap::Parser;
use std::env::VarError;
use std::error::Error;

const APP_NAME: &str = "clorange";
const DATA_DIR_ENV_VAR: &str = "CLORANGE_DATA_DIR";
const COUNTER_SUBCOMMAND_REQUIRED_ERROR: &str =
    "Counter file and subcommand are required, unless the --clear flag is specified.";

mod actions;
mod args;
mod data;

fn main() -> Result<(), Box<dyn Error>> {
    let Args {
        data,
        clear,
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
    if clear {
        actions::clear()?;
        return Ok(());
    }
    let Some(counter) = counter else {
        Err(COUNTER_SUBCOMMAND_REQUIRED_ERROR)?
    };
    let Some(action) = action else {
        Err(COUNTER_SUBCOMMAND_REQUIRED_ERROR)?
    };
    data::ensure_dirs_exist(&data)?;
    let counter = data.join(counter);
    data::ensure_file_exists(&counter)?;
    match action {
        Action::Increment => {
            actions::increment(counter)?;
        }
        Action::Decrement => {
            actions::decrement(counter)?;
        }
        Action::New => (), // we already ensured the file's existance
        Action::Reset => {
            actions::set(counter, 0)?;
        }
        Action::Set { num } => {
            actions::set(counter, num)?;
        }
        Action::Add { num } => {
            actions::add(counter, num)?;
        }
        Action::Subtract { num } => {
            actions::add(counter, -num)?;
        }
        Action::Show => {
            actions::show(counter)?;
        }
    }
    Ok(())
}
