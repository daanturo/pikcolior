<h1 align="center">
    Pikcolior: an very simple Linux CLI screen color picker
</h1>


# About

```
A simple CLI screen color picker that prints the color code.

Usage: pikcolior [OPTIONS]

Options:
  -c, --copy             Copy to the clipboard
  ...
```

Pikcolior works on Wayland. Just run the command to invoke the XDG color picker portal, then the #6-letter code will be printed. 

The `-c` option automatically copies the color code for you to paste somewhere else.

No fancy GUI, Pikcolior tries to stay out of your sight.


# Installing

## From source

Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

```bash
git clone --depth=1 https://gitlab.com/daanturo/pikcolior --branch stable
cd pikcolior

make

```


# Notable dependencies

Special thanks to those Rust library authors, I couldn't have created this tool without them:

- [ASHPD](https://docs.rs/ashpd/): Rust & zbus wrapper of the XDG portals DBus interfaces.
- [Arboard](https://docs.rs/arboard/): cross-platform library for interacting with the clipboard.
