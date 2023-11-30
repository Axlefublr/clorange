use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

mod about;

#[derive(Parser)]
#[command(author, version, long_about = about::LONG)]
pub struct Args {
    #[arg(short, long, value_name = "PATH", help = about::DATA)]
    pub data: Option<PathBuf>,
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    #[command(visible_alias = "inc")]
    Increment {
        #[arg(value_name = "PATH")]
        counter: PathBuf,
    },
    #[command(visible_alias = "dec")]
    Decrement {
        #[arg(value_name = "PATH")]
        counter: PathBuf,
    },
    #[command(visible_alias = "create", about = about::NEW)]
    New {
        #[arg(value_name = "PATH")]
        counter: PathBuf,
    },
    #[command(about = about::RESET)]
    Reset {
        #[arg(value_name = "PATH")]
        counter: PathBuf,
    },
    Set {
        #[arg(value_name = "PATH")]
        counter: PathBuf,
        num: i64,
    },
    Add {
        #[arg(value_name = "PATH")]
        counter: PathBuf,
        num: i64,
    },
    #[command(visible_alias = "sub")]
    Subtract {
        #[arg(value_name = "PATH")]
        counter: PathBuf,
        num: i64,
    },
    #[command(visible_alias = "see")]
    #[command(visible_alias = "look")]
    #[command(visible_alias = "view")]
    Show {
        #[arg(value_name = "PATH")]
        counter: PathBuf,
    },
    #[command(about = about::CLEAR)]
    Clear,
}
