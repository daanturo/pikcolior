use arboard::Clipboard;
use ashpd::desktop::Color;
use clap::{Parser, ValueEnum};
use futures::executor;
use std::process;

const HEX8_MAX: f64 = 0xFF as f64;
const HEX16_MAX: f64 = 0xFFFF as f64;

#[derive(Parser, Debug)]
#[command(version, about = "Pick screen color and print it.")]
struct Cli {
    /// Copy to the clipboard
    #[arg(short, long, default_value_t = false)]
    copy: bool,

    /// Print format, can be repeated to print multiple formats separated by newlines
    #[arg(short, long, value_delimiter = ',', default_values_t = vec![PrintFormat::Hex])]
    format: Vec<PrintFormat>,

    /// "u": Uppercase/#RRGGBB or "l": lowercase/#rrggbb
    #[arg(long, default_value_t = String::from("u"))]
    case: String,

    /// Rounding strategy when converting from the detected floating-point numbers, default: nearest
    #[arg(short, long, default_value_t = RoundStrategy::N)]
    round: RoundStrategy,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
enum PrintFormat {
    /// #RRGGBB
    Hex,
    /// #RRRRGGGGBBBB
    Hex16,
    /// R.float G.float B.float
    Float,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
enum RoundStrategy {
    /// Round to nearest
    N,
    /// Round up
    U,
    /// Round down
    D,
}

fn color_codes(color: (f64, f64, f64), cli: &Cli) -> Vec<String> {
    let (r, g, b) = color;
    let rounder = |num: f64| {
        (match cli.round {
            RoundStrategy::N => num.round(),
            RoundStrategy::U => num.ceil(),
            RoundStrategy::D => num.floor(),
        }) as u32
    };
    cli.format
        .iter()
        .map(|fmt| match fmt {
            PrintFormat::Hex => format!(
                "#{:02X}{:02X}{:02X}",
                rounder(r * HEX8_MAX),
                rounder(g * HEX8_MAX),
                rounder(b * HEX8_MAX),
            ),
            PrintFormat::Float => format!("{:?} {:?} {:?}", r, g, b),
            PrintFormat::Hex16 => format!(
                "#{:04X}{:04X}{:04X}",
                rounder(r * HEX16_MAX),
                rounder(g * HEX16_MAX),
                rounder(b * HEX16_MAX),
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
