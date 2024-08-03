<h1 align="center">
    Pikcolior: an extreme simple Linux CLI screen color picker
</h1>


# About

```
A simple CLI screen color picker that prints the RGB hex code.

Usage: pikcolior [OPTIONS]

Options:
  -c, --copy     Copy to the clipboard
```

Pikcolior works on Wayland. Just run the command to invoke the XDG color picker portal, then the #6-letter code will be printed. 

The `-c` option automatically copies the color code for you to paste somewhere else.

No fancy GUI, Pikcolior tries to stay out of your sight.


# Installing

Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

## Using cargo install

```bash
cargo install --git https://gitlab.com/daanturo/pikcolior --branch stable
```

## Add a launcher

Currently Pikcolior doesn't have its own icon. Create this file:

`~/.local/share/applications/Pikcolior.desktop` (you may need to make it executable `chmod u+x Pikcolior.desktop`)


```conf
[Desktop Entry]
Name=Pikcolior
GenericName=Screen color picker
Comment=Pick screen color and copy RGB hex code to the clipboard
Exec=pikcolior -c
Icon=color-picker
Type=Application
NoDisplay=false
Terminal=false
```

Launch and pin it on your Launcher/Dash/Start Menu/Taskbar for quick access, if you want.


# Notable dependencies

Special thanks to those Rust library authors, I couldn't have created this tool without them:

- [ASHPD](https://docs.rs/ashpd/): Rust & zbus wrapper of the XDG portals DBus interfaces.
- [Arboard](https://docs.rs/arboard/): cross-platform library for interacting with the clipboard.
