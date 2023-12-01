use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

mod about;

#[derive(Parser)]
#[command(author, version, long_about = about::LONG)]
pub struct Args {
    #[arg(short, long, value_name = "PATH", help = about::DATA)]
    pub data: Option<PathBuf>,
    #[arg(short, long, help = about::CLEAR)]
    pub clear: bool,
    #[arg(help = about::COUNTER)]
    pub counter: Option<PathBuf>,
    #[command(subcommand)]
    pub action: Option<Action>,
}

#[derive(Subcommand)]
pub enum Action {
    #[command(visible_alias = "inc")]
    #[command(visible_alias = "i")]
    Increment,
    #[command(visible_alias = "dec")]
    #[command(visible_alias = "d")]
    Decrement,
    #[command(visible_alias = "create", about = about::NEW)]
    #[command(visible_alias = "n")]
    New,
    #[command(visible_alias = "r")]
    #[command(about = about::RESET)]
    Reset,
    #[command(visible_alias = "t")]
    Set {
        num: i64,
    },
    #[command(visible_alias = "a")]
    Add {
        num: i64,
    },
    #[command(visible_alias = "s")]
    #[command(visible_alias = "sub")]
    Subtract {
        num: i64,
    },
    #[command(visible_alias = "v")]
    #[command(visible_alias = "see")]
    #[command(visible_alias = "look")]
    #[command(visible_alias = "view")]
    Show,
}
