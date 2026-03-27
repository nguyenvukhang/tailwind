use std::collections::HashMap;

use palette::{FromColor, OklabHue, Oklch, Srgb};
use serde_json::Value;

const SOURCE: &str = include_str!("../raw/colors.json");

/// Split `oklch(64.8% 0.2 131.684)` into ("64.8", "0.2", "131.684").
fn parse_oklch_str(oklch: &str) -> Option<(&str, &str, &str)> {
    let val = oklch.strip_prefix("oklch(")?;
    let val = val.strip_suffix(')')?;
    let (a, b) = val.split_once(' ')?;
    let (b, c) = b.split_once(' ')?;
    Some((a.strip_suffix('%')?, b, c))
}

fn parse() -> HashMap<String, HashMap<String, Oklch<f64>>> {
    let data: HashMap<String, Value> = serde_json::from_str(SOURCE).unwrap();
    let mut colors = data.keys().map(|v| v.as_str()).collect::<Vec<_>>();
    colors.retain(|k| data.get(*k).unwrap().is_object());

    let mut output = HashMap::new();

    for color in colors {
        let map = data[color].as_object().unwrap();
        let mut shades: Vec<_> = map.keys().map(|v| v.as_str()).collect();
        shades.sort_by_key(|v| v.parse::<u32>().unwrap());

        let mut color_map = HashMap::new();

        for shade in shades {
            let oklch_value = map.get(shade).unwrap().as_str().unwrap();
            let (a, b, c) = parse_oklch_str(oklch_value).unwrap();
            let a = a.parse::<f64>().unwrap() / 100.;
            let b = b.parse::<f64>().unwrap();
            let c = c.parse::<f64>().unwrap();

            let oklch = Oklch::new(a, b, c);
            let rgb = Srgb::from_color(oklch);

            let oklch2 = Oklch::new_const(a, b, OklabHue::from_degrees(c));
            let rgb2 = Srgb::from_color(oklch2);

            let oklch3 = Oklch::new_const(
                oklch.l,
                oklch.chroma,
                OklabHue::new(oklch.hue.into_raw_degrees()),
            );
            let rgb3 = Srgb::from_color(oklch3);

            assert_eq!(rgb, rgb2);
            assert_eq!(rgb, rgb3);
            color_map.insert(shade.to_string(), oklch2);
        }
        output.insert(color.to_string(), color_map);
    }
    output
}

fn cap1(x: &str) -> String {
    let (a, b) = x.split_at(1);
    format!("{}{}", a.to_uppercase(), b)
}

fn main() {
    let data = parse();

    println!("{}", "use super::*;");

    let mut n = 0;

    for (color, value) in &data {
        for (shade, oklch) in value {
            n += 1;
            let color_struct = format!("{}_{shade}", color.to_uppercase());
            print!("pub const {color_struct}: Color = Color {{");
            print!("name: \"{}{shade}\",", cap1(color));
            print!(
                "oklch: Oklch::new_const({a:?}, {b:?}, OklabHue::new({c:?}))",
                a = oklch.l,
                b = oklch.chroma,
                c = oklch.hue.into_raw_degrees(),
            );
            println!("}};");
        }
    }

    println!("pub const ALL_COLORS: [Color; {n}] = [");
    for (color, value) in &data {
        for shade in value.keys() {
            let color_struct = format!("{}_{shade}", color.to_uppercase());
            println!("{color_struct},");
        }
    }
    println!("];");
}
