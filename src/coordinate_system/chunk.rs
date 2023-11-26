use std::fmt::Display;
use std::path::MAIN_SEPARATOR;
use meridian_positioning::{GeoCoordinate, GeoRectangle};
use crate::coordinate_system::point::CoordinatePoint;
use crate::errors::Error;
use crate::heightmap;

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk
{
  pub center: GeoCoordinate,
  pub size: usize,             ///< in meters, side length
  pub folder: String,
  pub key: String
}

impl Default for Chunk
{
  fn default() -> Self
  {
    Self
    {
      center: GeoCoordinate::default(),
      size: 0,
      folder: std::env::current_dir().unwrap().to_str().unwrap().to_string(),
      key: format!("{}{}{}", "default", MAIN_SEPARATOR, "default")
    }
  }
}
impl Display for Chunk
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "({}, {}m)", self.center, self.size)
  }
}

impl Chunk
{
  pub fn new(center: GeoCoordinate, size: usize, zoom: u8, folder: &str) -> Result<Self, Error>
  {
    let key_tc = CoordinatePoint::from(center).tile_coordinate(zoom);
    let key = format!("{}{}{}", key_tc.0, MAIN_SEPARATOR, key_tc.1);
    let this = Self {
      center,
      size,
      folder: folder.to_string(),
      key
    };
    let p = this.path();
    std::fs::create_dir_all(p[..p.rfind(MAIN_SEPARATOR)
      .unwrap_or(p.len())]
      .to_string()
    )?;
    heightmap::convert_georectangle(
      &this.path(),
      GeoRectangle::from_center_meters(this.center, this.size as f32, this.size as f32)?,
      heightmap::Resolution::Low,
      heightmap::ImageFormat::PNG
    )?;
    Ok(this)
  }

  pub fn path(&self) -> String
  {
    format!("{}{MAIN_SEPARATOR}{}", self.folder, self.key)
  }
}