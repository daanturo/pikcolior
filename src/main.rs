use arboard::Clipboard;
use ashpd::desktop::Color;
use clap::Parser;
use futures::executor;
use std::cmp::min;

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
    let r_f = color.red();
    let g_f = color.green();
    let b_f = color.blue();
    (r_f, g_f, b_f)
}

fn main() {
    // 256
    let hex_ceil = 0x100 as f64;
    // 0xFF
    let hex_max = (hex_ceil - 1.0) as i16;

    let argv = Cli::parse();

    let color = pick_color();

    let r_i = min((color.0 * hex_ceil).round() as i16, hex_max);
    let g_i = min((color.1 * hex_ceil).round() as i16, hex_max);
    let b_i = min((color.2 * hex_ceil).round() as i16, hex_max);

    let color_code = format!("#{:02X}{:02X}{:02X}", r_i, g_i, b_i);

    println!("{}", color_code);

    if argv.copy {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(color_code.clone()).unwrap();
        // Staying alive a little bit after setting the clipboard
        clipboard.get_text().unwrap();
        assert_eq!(clipboard.get_text().unwrap(), color_code);
    }
}
