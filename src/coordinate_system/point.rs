use std::fmt::Display;
use meridian_positioning::positioning::GeoCoordinate;
use crate::coordinate_system::core::{project_to_web_mercator, TILE_SIZE};

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct CoordinatePoint
{
  pub x: f64,
  pub y: f64
}

impl Default for CoordinatePoint { fn default() -> Self { Self { x: 0.0, y: 0.0 } } }
impl Display for CoordinatePoint
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl From<GeoCoordinate> for CoordinatePoint
{
  fn from(coordinate: GeoCoordinate) -> Self
  {
    let t = project_to_web_mercator(coordinate.latitude, coordinate.longitude);
    Self { x: t.0, y: t.1 }
  }
}

impl CoordinatePoint
{
  pub fn new(x: f64, y: f64) -> Self { Self { x, y } }
  pub fn world_coordinate(&self) -> (f64, f64) { (self.x, self.y) }

  pub fn pixel_coordinate(&self, zoom_level: u8) -> (usize, usize)
  {
    let scale = CoordinatePoint::scale(zoom_level);
    ((self.x * scale as f64).floor() as usize, (self.y * scale as f64).floor() as usize)
  }

  pub fn tile_coordinate(&self, zoom_level: u8) -> (usize, usize)
  {
    let p = self.pixel_coordinate(zoom_level);
    (p.0 / TILE_SIZE, p.1 / TILE_SIZE)
  }

  fn scale(zoom_level: u8) -> i32
  {
    1 << zoom_level as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_web_mercator_world_coordinate()
  {
    let lat_lon = GeoCoordinate::new(41.85, -87.65, None);
    let point = CoordinatePoint::from(lat_lon);

    assert_eq!(point.world_coordinate(), (65.67111111111112, 95.17492654697409));
  }

  #[test]
  fn test_web_mercator_pixel_coordinate() {
    let lat_lon = GeoCoordinate::new(41.85, -87.65, None);
    let point = CoordinatePoint::from(lat_lon);

    assert_eq!(point.pixel_coordinate(0), (65, 95));
    assert_eq!(point.pixel_coordinate(1), (131, 190));
    assert_eq!(point.pixel_coordinate(2), (262, 380));
    assert_eq!(point.pixel_coordinate(3), (525, 761));
    assert_eq!(point.pixel_coordinate(4), (1050, 1522));
    assert_eq!(point.pixel_coordinate(5), (2101, 3045));
    assert_eq!(point.pixel_coordinate(6), (4202, 6091));
    assert_eq!(point.pixel_coordinate(7), (8405, 12182));
    assert_eq!(point.pixel_coordinate(8), (16811, 24364));
    assert_eq!(point.pixel_coordinate(9), (33623, 48729));
    assert_eq!(point.pixel_coordinate(10), (67247, 97459));
    assert_eq!(point.pixel_coordinate(11), (134494, 194918));
    assert_eq!(point.pixel_coordinate(12), (268988, 389836));
    assert_eq!(point.pixel_coordinate(13), (537977, 779672));
    assert_eq!(point.pixel_coordinate(14), (1075955, 1559345));
    assert_eq!(point.pixel_coordinate(15), (2151910, 3118691));
    assert_eq!(point.pixel_coordinate(16), (4303821, 6237383));
    assert_eq!(point.pixel_coordinate(17), (8607643, 12474767));
    assert_eq!(point.pixel_coordinate(18), (17215287, 24949535));
    assert_eq!(point.pixel_coordinate(19), (34430575, 49899071));
    assert_eq!(point.pixel_coordinate(20), (68861151, 99798143));
    assert_eq!(point.pixel_coordinate(21), (137722302, 199596287));
    assert_eq!(point.pixel_coordinate(22), (275444604, 399192575));
  }

  #[test]
  fn test_web_mercator_tile_coordinate() {
    let lat_lon = GeoCoordinate::new(41.85, -87.65, None);
    let point = CoordinatePoint::from(lat_lon);

    assert_eq!(point.tile_coordinate(0), (0, 0));
    assert_eq!(point.tile_coordinate(1), (0, 0));
    assert_eq!(point.tile_coordinate(2), (1, 1));
    assert_eq!(point.tile_coordinate(3), (2, 2));
    assert_eq!(point.tile_coordinate(4), (4, 5));
    assert_eq!(point.tile_coordinate(5), (8, 11));
    assert_eq!(point.tile_coordinate(6), (16, 23));
    assert_eq!(point.tile_coordinate(7), (32, 47));
    assert_eq!(point.tile_coordinate(8), (65, 95));
    assert_eq!(point.tile_coordinate(9), (131, 190));
    assert_eq!(point.tile_coordinate(10), (262, 380));
    assert_eq!(point.tile_coordinate(11), (525, 761));
    assert_eq!(point.tile_coordinate(12), (1050, 1522));
    assert_eq!(point.tile_coordinate(13), (2101, 3045));
    assert_eq!(point.tile_coordinate(14), (4202, 6091));
    assert_eq!(point.tile_coordinate(15), (8405, 12182));
    assert_eq!(point.tile_coordinate(16), (16811, 24364));
    assert_eq!(point.tile_coordinate(17), (33623, 48729));
    assert_eq!(point.tile_coordinate(18), (67247, 97459));
    assert_eq!(point.tile_coordinate(19), (134494, 194918));
    assert_eq!(point.tile_coordinate(20), (268988, 389836));
    assert_eq!(point.tile_coordinate(21), (537977, 779672));
    assert_eq!(point.tile_coordinate(22), (1075955, 1559345));
  }
}