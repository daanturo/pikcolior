use arboard::Clipboard;
use ashpd::desktop::Color;
use clap::{Parser, ValueEnum};
use futures::executor;
use std::process;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "A simple CLI screen color picker that prints the color code."
)]
struct Cli {
    /// Copy to the clipboard
    #[arg(short, long, default_value_t = false)]
    copy: bool,

    /// Print format
    #[arg(value_enum, short, long, default_value_t = PrintFormat::Hex)]
    format: PrintFormat,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum PrintFormat {
    /// #RRGGBB
    Hex,
    /// #RRRRGGGGBBBB
    Hex16,
    /// R.float G.float B.float
    Float,
}

fn pick_color() -> ashpd::Result<(f64, f64, f64)> {
    let color = executor::block_on(Color::pick().send())
        .unwrap()
        .response()?;
    let r_f = color.red();
    let g_f = color.green();
    let b_f = color.blue();
    Ok((r_f, g_f, b_f))
}

fn main() {
    let hex8_max = 0xFF as f64;
    let hex16_max = 0xFFFF as f64;
    let cli = Cli::parse();
    let color = pick_color();
    match color {
        Err(error) => {
            eprintln!("{error:?}");
            process::exit(0x1);
        }
        Ok(color) => {
            let printing = match cli.format {
                PrintFormat::Hex => format!(
                    "#{:02X}{:02X}{:02X}",
                    (color.0 * hex8_max).round() as u32,
                    (color.1 * hex8_max).round() as u32,
                    (color.2 * hex8_max).round() as u32,
                ),
                PrintFormat::Float => format!("{:?} {:?} {:?}", color.0, color.1, color.2),
                PrintFormat::Hex16 => format!(
                    "#{:04X}{:04X}{:04X}",
                    (color.0 * hex16_max).round() as u32,
                    (color.1 * hex16_max).round() as u32,
                    (color.2 * hex16_max).round() as u32,
                ),
            };

            println!("{}", printing);
            if cli.copy {
                let mut clipboard = Clipboard::new().unwrap();
                clipboard.set_text(printing.clone()).unwrap();
                // Staying alive a little bit after setting the clipboard
                clipboard.get_text().unwrap();
                assert_eq!(clipboard.get_text().unwrap(), printing);
            }
        }
    }
}
