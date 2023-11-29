use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::data;

pub fn clear() -> Result<(), io::Error> {
    let data_dir = data::default_location()?;
    fs::remove_dir_all(data_dir)?;
    Ok(())
}

pub fn increment(counter: PathBuf) -> Result<(), Box<dyn Error>> {
    data::read_write(&counter, |value| value + 1.0)
}

pub fn decrement(counter: PathBuf) -> Result<(), Box<dyn Error>> {
    data::read_write(&counter, |value| value - 1.0)
}

pub fn set(counter: PathBuf, num: f64) -> Result<(), Box<dyn Error>> {
    Ok(data::write(&counter, num)?)
}

pub fn add(counter: PathBuf, num: f64) -> Result<(), Box<dyn Error>> {
    data::read_write(&counter, |value| value + num)
}

pub fn show(counter: PathBuf) -> Result<(), Box<dyn Error>> {
    Ok(println!("{}", data::read(&counter)?))
}
