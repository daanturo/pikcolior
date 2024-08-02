use ashpd::desktop::Color;
use futures::executor;

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
    let color = pick_color();
    let max_val = 0x100 as f64;
    let R = (color.0 * max_val).round() as i16;
    let G = (color.1 * max_val).round() as i16;
    let B = (color.2 * max_val).round() as i16;
    let color_code = format!("#{:02X}{:02X}{:02X}", R, G, B);
    println!("{}", color_code);
}
