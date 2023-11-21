use about::*;
use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

mod about;

#[derive(Parser)]
#[command(author, version, long_about = ABOUT)]
pub struct Args {
    #[arg(short, long, value_name = "PATH", help = DATA_ABOUT)]
    pub data: Option<PathBuf>,
    #[arg(help = COUNTER_ABOUT)]
    pub counter: PathBuf,
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    #[command(visible_alias = "inc")]
    Increment,
    #[command(visible_alias = "dec")]
    Decrement,
    #[command(visible_alias = "create", about = NEW_ABOUT)]
    New,
    Reset,
    Set {
        num: f64,
    },
    Add {
        num: f64,
    },
    #[command(visible_alias = "sub")]
    Subtract {
        num: f64,
    },
    #[command(visible_alias = "see")]
    #[command(visible_alias = "look")]
    #[command(visible_alias = "view")]
    Show,
    #[command(about = CLEAR_ABOUT)]
    Clear,
}
