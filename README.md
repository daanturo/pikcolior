<h1 align="center">
    Pikcolior: an very simple Linux CLI screen color picker
</h1>


# About

Run `pikcolior` to invoke the XDG color picker portal, then the #6-letter code will be printed. 

The `-c` option automatically copies the color code to your clipboard.

See more options by invoking [`pikcolior --help`](./target/doc/help-output).

No fancy GUI, Pikcolior tries to stay out of your sight.


# Installing

## From source

Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

```bash
git clone --depth=1 https://gitlab.com/daanturo/pikcolior --branch stable
cd pikcolior

make install

```

The above installs `pikcolior` to your [$CARGO_HOME/bin](https://doc.rust-lang.org/cargo/guide/cargo-home.html#directories) and a desktop entry to `~/.local/share/applications/`.

# Notable dependencies

Special thanks to those Rust library authors, I couldn't have created this tool without them:

- [ASHPD](https://docs.rs/ashpd/): Rust & zbus wrapper of the XDG portals DBus interfaces.
- [Arboard](https://docs.rs/arboard/): cross-platform library for interacting with the clipboard.
