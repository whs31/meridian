use std::f64::consts::{E, PI};
use std::ops::Div;

const THRESHOLD: f64 = 0.9999;
pub const TILE_SIZE: usize = 256;

pub fn project_to_web_mercator(lat: f64, lon: f64) -> (f64, f64)
{
  (
    TILE_SIZE as f64 * (0.5 + lon / 360.0),
    TILE_SIZE as f64 * (0.5 -
      conjugate_ratio(1.0, lat
        .to_radians()
        .sin()
        .max(-THRESHOLD)
        .min(THRESHOLD)
      )
      .log(E)
      .div(4.0 * PI)
    )
  )
}

fn conjugate_ratio(x: f64, b: f64) -> f64 { (x + b) / (x - b) }