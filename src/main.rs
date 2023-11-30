#![deny(unused_must_use)]

use args::Action;
use args::Args;
use clap::Parser;
use std::env::VarError;
use std::error::Error;

const APP_NAME: &str = "clorange";
const DATA_DIR_ENV_VAR: &str = "CLORANGE_DATA_DIR";

mod actions;
mod args;
mod data;

fn main() -> Result<(), Box<dyn Error>> {
    let Args { data, action } = Args::parse();
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
    if let Action::Clear = action {
        actions::clear()?;
        return Ok(());
    }
    data::ensure_dirs_exist(&data)?;
    match action {
        Action::Increment { counter } => {
            let counter = data.join(counter);
            data::ensure_file_exists(&counter)?;
            actions::increment(data.join(counter))?;
        }
        Action::Decrement { counter } => {
            let counter = data.join(counter);
            data::ensure_file_exists(&counter)?;
            actions::decrement(data.join(counter))?;
        },
        Action::New { counter } => data::ensure_file_exists(&data.join(counter))?,
        Action::Reset { counter } => {
            let counter = data.join(counter);
            data::ensure_file_exists(&counter)?;
            actions::set(data.join(counter), 0.0)?;
        },
        Action::Set { counter, num } => {
            let counter = data.join(counter);
            data::ensure_file_exists(&counter)?;
            actions::set(data.join(&counter), num)?;
        },
        Action::Add { counter, num } => {
            let counter = data.join(counter);
            data::ensure_file_exists(&counter)?;
            actions::add(data.join(&counter), num)?;
        },
        Action::Subtract { counter, num } => {
            let counter = data.join(counter);
            data::ensure_file_exists(&counter)?;
            actions::add(data.join(&counter), -num)?;
        },
        Action::Show { counter } => {
            let counter = data.join(counter);
            data::ensure_file_exists(&counter)?;
            actions::show(data.join(&counter))?;
        }
        Action::Clear => unreachable!("Already short circuited prior")
    }
    Ok(())
}
