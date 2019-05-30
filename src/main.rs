use std::io;
use std::io::Write;

use palette::{Color, Shade, Saturate, Hue};
use palette::rgb::{Rgb, Srgb};
use rand::prelude::*;
use termcolor;
use termcolor::{WriteColor};

const SPARKLES: &str = include_str!("../data/sparkles.txt");

fn rand_unit() -> f32 {
    thread_rng().gen_range(-1.0, 1.0)
}

fn similar_to(color: Color, amount: f32) -> Color {
    color.lighten(rand_unit() * amount)
        .shift_hue(rand_unit() * amount)
        .saturate(rand_unit() * amount)
}

fn palette_to_termcolor(color: Color) -> termcolor::Color {
    let rgb: Rgb<_, u8> = Into::<Srgb>::into(color).into_format();
    termcolor::Color::Rgb(rgb.red, rgb.green, rgb.blue)
}

fn sparkles(len: usize, sparkles: Vec<char>) -> String {
    if sparkles.is_empty() {
        return "".into();
    }
    let mut s = String::with_capacity(len);
    let mut rng = thread_rng();
    for _ in 0..len {
        s.push(*sparkles.choose(&mut rng).unwrap());
    }
    s
}

fn colored_str(color: Color, spread: f32, s: &str)
-> Result<Box<termcolor::Buffer>, io::Error> {
    let mut buf = termcolor::Buffer::ansi();
    for c in s.chars() {
        buf.set_color(termcolor::ColorSpec::new()
                .set_fg(Some(palette_to_termcolor(similar_to(color, spread)))))?;
        write!(&mut buf, "{}", c)?;
    }
    Ok(Box::new(buf))
}

fn colored_sparkles<'a>(color: Color, len: usize, spread: f32)
-> Result<Box<termcolor::Buffer>, io::Error> {
    colored_str(color, spread, &sparkles(len, SPARKLES.chars().collect()))
}

fn main() -> Result<(), io::Error> {
    let base = Color::linear_rgb(0.0, 0.75, 1.0);
    io::stdout().write(colored_sparkles(base, 16, 0.3)?.as_slice())?;
    io::stdout().write(b" ")?;
    io::stdout().write(colored_str(base, 0.1, "squeaky clean!")?.as_slice())?;
    io::stdout().write(b" ")?;
    io::stdout().write(colored_sparkles(base, 16, 0.3)?.as_slice())?;
    println!();
    Ok(())
}
