# Clorange

The idea of the project is to give you a simple to use shell oriented counter.

(The name comes from Clockwork Orange. Idk "clockwork" sounds kinda like counting.)

Have you ever built a simple function, which ended up being unecessarily complex because you wanted to count something?

Let me give you an example.

I have this fish function which functions (lol and even lmao) as a pomodorro timer. It's roughly this:

```fish
function pom
    while true
        termdown 25m || break
        termdown 5m || break
    end
end
```

Would be nice to know at any time how many work periods I've done, right?

That brings us to this horrible expression, that would increment a number stored in a file every time a period is done: `printf (math (cat /tmp/workie) + 1) > /tmp/workie`

Truly horrifying! Sure, maybe it's a function you make and never look at again, but you have no convenient ways to interact with the number.

I absolutely hate typing in full paths, and right now to change / look at the value I'd need to `nvim /tmp/workie` or `cat /tmp/workie`.

Sure, I could just make an alias for those two specific commands, but this is just sweeping the issue under the rug, because I can have potentially infinite things to count, creating infinite files, and therefore aliases.

To add, instead of having nice abstractions such as "increment", "decrement", etc, you'd have to now *type out* `printf (math (cat /tmp/workie) + 1) > /tmp/workie`, which I sincerely hope you wouldn't want to do.

This program solves this by giving you a nice, clean interface to modify singular counter files.

No need to preemptively create files: clorange will do that for you.

Clean and simple subcommands: `increment`, `decrement`, `add`, `sub`, `show` — and they do exactly what you think they do.

You don't even have to type them out! All the subcommands have convenient aliases, which you'll be able to learn about in the [Usage](#Usage) section.

No need to edit the files manually when you need to set arbitrary values — use `clorange instances set 69` to write the number 69 into the counter called "instances".

Want to ensure a counter file exists for a laugh? We have a tool for that, it's called `clorange instances new`.

## Usage

```
By default, clorange stores counters in a directory named 'clorange' located in
your OS' data directory.
* Linux: /home/username/.local/share/clorange
* MacOS: /Users/username/Library/Application Support/clorange
* Windows: C:/Users/username/AppData/Roaming/clorange
You can override it by setting the environment variable CLORANGE_DATA_DIR to the
path you want.
An even stronger override is using the -d / --data flags.
Both don't require you to create the subdirectories in the path beforehand.

Each counter uses its own file (which contains a single number), that you modify
with subcommands.
The default value in a new counter file is 0.
Counter files are created automatically once you call any subcommand.
You can specify parent directories in the <COUNTER> argument, and they also don't
have to exist beforehand.
The relative-looking path is effectively relative to the data directory.

Usage: clorange [OPTIONS] [COUNTER] [COMMAND]

Commands:
  increment  [aliases: inc, i]
  decrement  [aliases: dec, d]
  new        Ensures that the counter file exists, and creates it if it
                 doesn't [aliases: create, n]
  reset      Set the counter to 0. Acts as an equivalent alias to `set 0`
                 [aliases: r]
  set        [aliases: t]
  add        [aliases: a]
  subtract   [aliases: s, sub]
  show       [aliases: v, see, look, view]
  help       Print this message or the help of the given subcommand(s)

Arguments:
  [COUNTER]  Specify a path to a counter file, relative to the data directory

Options:
  -d, --data <PATH>  Specify the directory to make the counter files in
  -c, --clear        Delete the default data directory along with all the counter
                     files, and then immediately exit. You would usually do this
                     before uninstalling clorange D: Or to just clean up old
                     unused counters :D"
  -h, --help         Print help
  -V, --version      Print version
```

## Install

```
cargo install clorange
```

`cargo-binstall` and `cargo-quickinstall` are also supported.

## Uninstall

```
clorange --clear
cargo uninstall clorange
```