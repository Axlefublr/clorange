use crate::APP_NAME;
use crate::DATA_DIR_ENV_VAR;
use const_format::formatcp;

pub const LONG: &str = formatcp!("\
By default, {APP_NAME} stores counters in a directory named '{APP_NAME}' located in your OS' data directory.
  * Linux: /home/username/.local/share/clorange
  * MacOS: /Users/username/Library/Application Support/clorange
  * Windows: C:/Users/username/AppData/Roaming/clorange
You can override it by setting the environment variable {DATA_DIR_ENV_VAR} to the path you want.
An even stronger override is using the -d / --data flags.
Both don't require you to create the subdirectories in the path beforehand.

Each counter uses its own file (which contains a single number), that you modify with subcommands.
The default value in a new counter file is 0.
Counter files are created automatically once you call any subcommand.
You can specify parent directories in the <COUNTER> argument, and they also don't have to exist beforehand.
The relative-looking path is effectively relative to the data directory.");

pub const DATA: &str = "Specify the directory to make the counter files in.";

pub const CLEAR: &str = formatcp!(
    "\
Delete the default data directory along with all the counter files,
and then immediately exit.
You would usually do this before uninstalling {APP_NAME} D:
Or to just clean up old unused counters :D"
);

pub const COUNTER: &str = "Specify a path to a counter file, relative to the data directory.";

pub const NEW: &str = "\
Ensures that the counter file exists,
and creates it if it doesn't.";

pub const RESET: &str = "\
Set the counter to 0.
Acts as an equivalent alias to `set 0`";
