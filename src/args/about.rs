use crate::APP_NAME;
use crate::DATA_DIR_ENV_VAR;
use const_format::formatcp;

pub const ABOUT: &str = formatcp!("\
Clorange will first pick its data directory, where it will store the counter files.
Each counter uses its own file.
It's done this way (instead of a singular json file, for example) so you can more easily integrate the counter files into your workflows.
Maybe you have another program that depends on a counter file: it would be far more annoying to *have to* use clorange to read it.

By default, clorange makes a directory named \"{APP_NAME}\" in your OS' data directory to store the counters.
Linux: /home/username/.local/share/clorange
MacOS: /Users/username/Library/Application Support/clorange
Windows: C:/Users/username/AppData/Roaming/clorange
You can override it by setting the environment variable {DATA_DIR_ENV_VAR} to the path you want.
An even stronger override is using the -d / --data flags.
Both don't require you to create the subdirectories in the path beforehand.

Each counter file contains a single number, that you modify with subcommands.
The number is a float, so float operations are supported, but you can just specify your argument numbers as integers (1 vs 1.0)");

pub const DATA_ABOUT: &str = formatcp!(
    "\
Specify the directory to make the counter files in.
Subdirectories don't have to exist already, {APP_NAME} will make them for you.
The data directory acts as a \"current working directory\" essentially,
since you can still specify paths in subdirectories for the counter file."
);

pub const COUNTER_ABOUT: &str = formatcp!(
    "\
Specify the filename of a counter you want to (possibly) modify.
Counter files, along with all their parent directories, are created automatically
if the subcommand you're calling isn't \"clear\".
This is useful in such a way that you can consider the data directory as a 'current working directory',
to which you can then specify relative paths in this argument like 'subdir/reboots',
keeping your counter files easily organized, without having to change the data directory all the time.
The default value in a new counter file is 0."
);

pub const CLEAR_ABOUT: &str = formatcp!(
    "\
Delete the default data directory along with all the counter files.
You would usually do this before uninstalling {APP_NAME} D:
Or to just clean up old unused counters :D"
);

pub const NEW_ABOUT: &str = "\
Simply ensures that the counter file exists, and if it doesn't, creates it with the default value of 0.
The difference between this and `reset` or `set 0` is that if the file already exists, nothing will happen.
With the other two mentioned subcommands, an additional write will be done to the file, whether or not
it already exists; ensuring that that counter's value is 0.
You can use any subcommand without preemptively creating the counter file,
so this subcommand will mostly be useless.";
