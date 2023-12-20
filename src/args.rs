use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

mod about {
    use indoc::indoc;
    pub const ABOUT: &str = indoc! {"
        By default, clorange stores counters in a directory named 'clorange' located in your OS' data directory.
        * Linux: /home/username/.local/share/clorange
        * MacOS: /Users/username/Library/Application Support/clorange
        * Windows: C:/Users/username/AppData/Roaming/clorange
        You can override it by setting the environment variable CLORANGE_DATA_DIR to the path you want.
        An even stronger override is using the -d / --data flags.
        Both don't require you to create the subdirectories in the path beforehand.

        Each counter uses its own file (which contains a single number), that you modify with subcommands.
        The default value in a new counter file is 0.
        Counter files are created automatically once you call any subcommand.
        You can specify parent directories in the <COUNTER> argument, and they also don't have to exist beforehand.
        The relative-looking path is effectively relative to the data directory.
    "};
}

#[derive(Parser)]
#[command(author, version, about = about::ABOUT)]
pub struct Args {
    /// Specify the directory to make the counter files in.
    #[arg(short, long, value_name = "PATH")]
    pub data: Option<PathBuf>,
    /// Delete the default data directory along with all the counter files, and then immediately exit.
    /// You would usually do this before uninstalling clorange D:
    /// Or to just clean up old unused counters :D"
    #[arg(short, long)]
    pub clear: bool,
    /// Specify a path to a counter file, relative to the data directory.
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
    /// Ensures that the counter file exists, and creates it if it doesn't.
    #[command(visible_alias = "create")]
    #[command(visible_alias = "n")]
    New,
    /// Set the counter to 0. Acts as an equivalent alias to `set 0`.
    #[command(visible_alias = "r")]
    Reset,
    #[command(visible_alias = "t")]
    Set { num: i64 },
    #[command(visible_alias = "a")]
    Add { num: i64 },
    #[command(visible_alias = "s")]
    #[command(visible_alias = "sub")]
    Subtract { num: i64 },
    #[command(visible_alias = "v")]
    #[command(visible_alias = "see")]
    #[command(visible_alias = "look")]
    #[command(visible_alias = "view")]
    Show,
}
