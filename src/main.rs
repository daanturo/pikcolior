use arboard::Clipboard;
use ashpd::desktop::Color;
use clap::Parser;
use futures::executor;

#[derive(Parser, Debug)]
#[command(version, about = "Simple CLI color picker that prints RGB hex code.")]
struct Cli {
    /// Copy to the clipboard
    #[arg(short, long, default_value_t = false)]
    copy: bool,
}

fn pick_color() -> (f64, f64, f64) {
    let color = executor::block_on(Color::pick().send())
        .unwrap()
        .response()
        .unwrap();
    let r = color.red();
    let g = color.green();
    let b = color.blue();
    (r, g, b)
}

fn main() {
    // 256
    let max_val = 0x100 as f64;

    let argv = Cli::parse();

    let color = pick_color();
    let R = (color.0 * max_val).round() as i16;
    let G = (color.1 * max_val).round() as i16;
    let B = (color.2 * max_val).round() as i16;
    let color_code = format!("#{:02X}{:02X}{:02X}", R, G, B);

    println!("{}", color_code);

    if argv.copy {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(color_code.clone()).unwrap();
        // Staying alive a little bit after setting the clipboard
        clipboard.get_text().unwrap();
        assert_eq!(clipboard.get_text().unwrap(), color_code);
    }
}
