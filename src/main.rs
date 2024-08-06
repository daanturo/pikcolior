use arboard::Clipboard;
use ashpd::desktop::Color;
use clap::{Parser, ValueEnum};
use futures::executor;
use std::process;

const HEX8_MAX: f64 = 0xFF as f64;
const HEX16_MAX: f64 = 0xFFFF as f64;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "A simple CLI screen color picker that prints the color code."
)]
struct Cli {
    /// Copy to the clipboard
    #[arg(short, long, default_value_t = false)]
    copy: bool,

    /// Print format, can be repeated to print multiple formats separated by newlines
    #[arg(short, long, default_values_t = vec![PrintFormat::Hex])]
    format: Vec<PrintFormat>,

    /// Uppercase ("u") or lowercase ("l")
    #[arg(long, default_value_t = String::from("u"))]
    case: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, strum_macros::Display)]
enum PrintFormat {
    /// #RRGGBB
    #[strum(to_string = "hex")]
    Hex,
    /// #RRRRGGGGBBBB
    #[strum(to_string = "hex16")]
    Hex16,
    /// R.float G.float B.float
    #[strum(to_string = "float")]
    Float,
}

fn color_codes(color: (f64, f64, f64), cli: &Cli) -> Vec<String> {
    cli.format
        .iter()
        .map(|fmt| match fmt {
            PrintFormat::Hex => format!(
                "#{:02X}{:02X}{:02X}",
                (color.0 * HEX8_MAX).round() as u32,
                (color.1 * HEX8_MAX).round() as u32,
                (color.2 * HEX8_MAX).round() as u32,
            ),
            PrintFormat::Float => format!("{:?} {:?} {:?}", color.0, color.1, color.2),
            PrintFormat::Hex16 => format!(
                "#{:04X}{:04X}{:04X}",
                (color.0 * HEX16_MAX).round() as u32,
                (color.1 * HEX16_MAX).round() as u32,
                (color.2 * HEX16_MAX).round() as u32,
            ),
        })
        .map(|str| {
            if cli.case == "l" {
                str.to_lowercase()
            } else {
                str
            }
        })
        .collect()
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
    let cli = Cli::parse();
    if !["u", "l"].contains(&cli.case.as_str()) {
        eprintln!("Invalid argument '{}' for '--case'.", cli.case);
        process::exit(0x1);
    }
    let color = pick_color();
    match color {
        Err(error) => {
            eprintln!("{error:?}");
            process::exit(0x1);
        }
        Ok(color) => {
            let codes = color_codes(color, &cli);
            let printing = codes.join("\n");

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
