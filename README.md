# Advent of Code 2023

2023 didn't go so well for me. It really didn't go at all. I did my first powerlifting meet on 2 December 2023.
This obviously interfered with day 2, but it also messed me up for day 3, and somehow I just couldn't get myself
over that motivation slump.

At the time, I had the best of intentions to practice Java in 2023.

Well, now it's 2025 and I'm using Rust instead.

# Daily Stars and Themes

1. `##`
2. `**`
3. `**`
4. `**`
5. `**`
6. `**` Calculus, worse-is-better
7. `**` Comparators, refactoring, finite-state machines
8. `**` Repeated loops (`cycle`), LCM (`num` crate)
9. `**` Sequences, [differencing](https://otexts.com/fpp2/stationarity.html), in-place modification, triangular numbers
10. `**` Paths, areas, BFS, right-hand rule from physics
11. `**` Manhattan distance
12. `**` Parsers, dynamic programming

# Lessons learned
- `cargo check` is a very fast way to check if your code would compile.
- I've been reaching for the object-oriented approach more and more. It's becoming more convenient to
represent the puzzle as a large object, rather than parsing the input in a `part1` function.
- See day 10 for examples of implementing the `Add`, `Mul`, and `Display` traits.

# Libraries
- [`num`](https://crates.io/crates/num)
- [Zelen](https://crates.io/crates/zelen), a MiniZinc frontend for [Selen](https://github.com/radevgit/selen), a CSP solver.

Writing this down here for posterity. (I know this isn't Rust.) Check out [Ghostty](https://ghostty.org) as an alternative terminal emulator for Linux and Mac.

# References

- [A Comparison of Ada and Rust, using solutions to the Advent of Code](https://github.com/johnperry-math/AoC2023/blob/master/More_Detailed_Comparison.md)
- [Automatically Translating C to Rust](https://cacm.acm.org/research/automatically-translating-c-to-rust/)
- [My MiniZinc page, by Hakan K.](https://www.hakank.org/minizinc/) (yes, I know this isn't Rust)
- [Fast UDP I/O for Firefox in Rust](https://max-inden.de/post/fast-udp-io-in-firefox/)
- [Humanity is stained by the sins of C and no LLM can rewrite them away to Rust](https://kirancodes.me/posts/log-sins-of-c.html)
- [Patterns for Defensive Programming in Rust](https://corrode.dev/blog/defensive-programming/)
- [Cloudflare outage on November 18, 2025](https://blog.cloudflare.com/18-november-2025-outage/)
- [Using unwrap() in Rust is Okay](https://burntsushi.net/unwrap/)
- [Linus Torvalds Vents Over "Completely Crazy Rust Format Checking"](https://www.phoronix.com/news/Linus-Torvalds-Rust-Formatting)

# Emacs-style keybindings for Zed

I'm trying Zed as a Rust IDE. I like that it's fast, and I like that it supports Emacs-style keybindings.
However, I find myself in a strange position where I know enough Emacs-style keystrokes to enjoy those
but not enough to benefit from the ones I don't know. I like CTRL+C/CTRL+V. I also like ALT+D to delete
the current word. So, I decided to just do both. I'm using the VSCode-style keybindings base with the
following customizations.

```
  {
    "context": "Editor",
    "bindings": {
      "alt-backspace": [
        "editor::DeleteToPreviousWordStart",
        {
          "ignore_newlines": false,
          "ignore_brackets": false
        }
      ]
    }
  },
  {
    "context": "Editor",
    "bindings": {
      "ctrl-d": "editor::Delete"
    }
  },
  {
    "context": "Editor",
    "bindings": {
      "ctrl-a": [
        "editor::MoveToBeginningOfLine",
        {
          "stop_at_soft_wraps": true,
          "stop_at_indent": true
        }
      ]
    }
  },
  {
    "context": "Editor",
    "bindings": {
      "ctrl-x h": "editor::SelectAll"
    }
  },
  {
    "context": "Editor",
    "bindings": {
      "ctrl-e": [
        "editor::MoveToEndOfLine",
        {
          "stop_at_soft_wraps": true
        }
      ]
    }
  },
  {
    "context": "Editor",
    "bindings": {
      "alt-shift-.": "editor::MoveToEnd"
    }
  },
  {
    "context": "Editor",
    "bindings": {
      "alt-shift-,": "editor::MoveToBeginning"
    }
  },
  {
    "context": "Editor",
    "bindings": {
      "ctrl-n": "editor::MoveDown"
    }
  },
  {
    "context": "Editor",
    "bindings": {
      "ctrl-p": "editor::MoveUp"
    }
  },
  {
    "context": "Editor",
    "bindings": {
      "ctrl-f": "editor::MoveRight"
    }
  },
  {
    "context": "Editor",
    "bindings": {
      "ctrl-b": "editor::MoveLeft"
    }
  },
  {
    "context": "Editor",
    "bindings": {
      "alt-f": "editor::MoveToNextWordEnd"
    }
  },
  {
    "context": "Editor",
    "bindings": {
      "alt-b": "editor::MoveToPreviousWordStart"
    }
  },
  {
    "context": "Editor",
    "bindings": {
      "alt-d": [
        "editor::DeleteToNextWordEnd",
        {
          "ignore_newlines": false,
          "ignore_brackets": false
        }
      ]
    }
  },
  {
    "context": "Workspace",
    "bindings": {
      "ctrl-x ctrl-s": "workspace::Save"
    }
  }
```
