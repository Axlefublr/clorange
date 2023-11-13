# Clorange (NOT FINISHED)

The idea of the project is to give you a simple to use shell oriented counter.

(The name comes from Clockwork Orange. Idk "clockwork" sounds kinda like counting.)

Have you ever built a simple function, which ended up being unecessarily complex because you wanted to count something?

Let me give you an example.

I have this fish function which functions (lol and even lmao) as a pomodorro timer.

```fish
function work
	while true
		kitten @set-window-title work
		termdown 25m || break
		echo "You just worked! Rest now?"
		read -ln 1 should_continue
		if not test $should_continue
			break
		end
		clear

		kitten @set-window-title rest
		termdown 5m || break
		echo "You just rested! Work now?"
		read -ln 1 should_continue
		if not test $should_continue
			break
		end
		clear
	end
	clear
	kitten @set-window-title
end
```

Would be nice to know at any time how many work periods I've done, right?

That brings us to this horrible expression, that would increment a number stored in a file every time a period is done: `printf (math (cat /tmp/workie) + 1) > /tmp/workie`

Truly horrifying! Sure, maybe it's a function you make and never look at again, but you have no convenient ways to interact with the number.

I absolutely hate typing in full paths, and right now to change / look at the value I'd need to `nvim /tmp/workie` or `cat /tmp/workie`.

Sure, I could just make an alias for those two specific commands, but this is just sweeping the issue under the rug, because I can have a potentially infinite things to count, creating infinite files, and therefore aliases.

To add, instead of having nice abstractions such as "increment", "decrement", etc, you'd have to now *type out* `printf (math (cat /tmp/workie) + 1) > /tmp/workie`, which I sincerely hope you wouldn't want to do.

This program blah blah blah...