use std::collections::{BTreeMap};
use std::fmt::{Display, Error};

pub type ShadeUKey = u32;

#[derive(Eq, Ord, PartialOrd, PartialEq, Copy, Clone, Debug)]
pub enum ShadeKey {
  U(ShadeUKey),
  Default
}

impl Display for ShadeKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
    match self {
      ShadeKey::U(shade_key) => write!(f, "{}", shade_key),
      ShadeKey::Default => write!(f, "DEFAULT")
    }
  }
}

impl ShadeKey {
  pub fn shades() -> Vec<ShadeKey> {
    vec![
      ShadeKey::Default,
      ShadeKey::U(50),
      ShadeKey::U(100),
      ShadeKey::U(200),
      ShadeKey::U(300),
      ShadeKey::U(400),
      ShadeKey::U(500),
      ShadeKey::U(600),
      ShadeKey::U(700),
      ShadeKey::U(800),
      ShadeKey::U(900),
      ShadeKey::U(950),
    ]
  }
}

fn has_prefix(s: &str, prefix: &str) -> bool {
  s.starts_with(prefix)
}
pub fn is_valid_hex(hex: &str) -> bool {
  (hex.starts_with("#") && hex.chars().skip(1).all(|c| c.is_digit(16)))
    || hex.chars().all(|c| c.is_digit(16))
}
pub fn shades_of(hex: &str) -> Result<BTreeMap<ShadeKey, String>, String> {
  if !is_valid_hex(hex) {
    return Err(format!("Invalid hex: {}", hex));
  }
  let mut hex_mut = hex.to_string();

  if !has_prefix(hex_mut.as_str(), "#") {
    hex_mut = format!("#{}", hex_mut);
  }
  let base_color = hex_to_rgb_array(hex_mut.as_str());
  let black: Vec<u8> = vec![0, 0, 0];
  let white: Vec<u8> = vec![255, 255, 255];

  let shades: Vec<ShadeUKey> = vec![50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

  let mut result = BTreeMap::new();
  if !result.contains_key(&ShadeKey::Default) {
    result.insert(ShadeKey::Default, hex_mut.to_string());
  }

  for shade in shades.iter() {
    if *shade == 500 {
      result.insert(ShadeKey::U(500), hex_mut.to_string());
    } else {
      if *shade < 500 {
        let percentage = (500 - shade) as f32 / 500.0;
        result.insert(ShadeKey::U(*shade), get_color(percentage, white.clone(), base_color.clone()));
      } else {
        let percentage = (shade - 500) as f32 / 500.0;
        result.insert(ShadeKey::U(*shade), get_color(percentage, black.clone(), base_color.clone()));
      }
    }
  }

  Ok(result)
}

fn hex_to_rgb_array(hex: &str) -> Vec<u8> {
  let mut hex = String::from(hex);

  hex = hex.replace("#", "");

  if hex.len() == 3 {
    let mut new_hex = String::new();
    for c in hex.chars() {
      new_hex.push_str(&c.to_string().repeat(2));
    }
    hex = new_hex;
  }

  let (r, gb) = hex.split_at(2);
  let (g, b) = gb.split_at(2);

  let vec_channels = vec![r.to_string(), g.to_string(), b.to_string()];

  let vec_int_channels: Vec<u8> = vec_channels.iter().map(|channel| {
    u8::from_str_radix(channel, 16).unwrap()
  }).collect::<Vec<_>>();

  vec_int_channels
}

fn get_color(percentage: f32, start: Vec<u8>, end: Vec<u8>) -> String {
  let rgb = end.iter().enumerate().map(|(index, channel)| {
    let rgb_channel = (*channel as f32) + percentage * ((start[index] as f32) - (*channel as f32));
    rgb_channel.round()
  });

  let hex_channel = rgb.map(|ch| {
    format!("{:02x}", ch as u8)
  }).collect::<String>();

  format!("#{}", hex_channel)
}