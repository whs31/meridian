use std::fmt::Display;
use std::path::MAIN_SEPARATOR;
use crate::positioning::geocoordinate::GeoCoordinate;
use crate::tile_storage::quarter::Quarter;

pub static EXTENSION: &'static str = "tif";

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct TileSignature
{
  pub latitude: i8,
  pub longitude: i16
}

impl Display for TileSignature
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "[{}, {} ({:?})]",
           self.latitude,
           self.longitude,
           self.quarter()
    )
  }
}

impl TileSignature
{
  #[allow(dead_code)]
  pub fn new(latitude: i8, longitude: i16) -> Self
  {
    Self {
      latitude,
      longitude
    }
  }

  pub fn from_f64(latitude: f64, longitude: f64) -> Self
  {
    Self {
      latitude: latitude.floor() as i8,
      longitude: longitude.floor() as i16
    }
  }

  pub fn quarter(&self) -> Quarter
  {
    if self.latitude >= 0 && self.longitude < 0 { return Quarter::TopLeft }
    if self.latitude >= 0 && self.longitude >= 0 { return Quarter::TopRight }
    return if self.latitude < 0 && self.longitude < 0 { Quarter::BottomLeft } else { Quarter::BottomRight }
  }

  pub fn to_relative_path(&self) -> String
  {
    return format!("{}{MAIN_SEPARATOR}{}{MAIN_SEPARATOR}{}.{EXTENSION}",
                   self.quarter().to_u8(),
                   self.latitude.abs(),
                   self.longitude.abs()
    ).to_string()
  }

  pub fn georectangle_size(&self) -> (usize, usize)
  {
    (
      GeoCoordinate::new(self.latitude as f64, self.longitude as f64, 0.0)
        .distance_to(&GeoCoordinate::new((self.latitude + 1) as f64, self.longitude as f64, 0.0))
        .unwrap() as usize,
      GeoCoordinate::new(self.latitude as f64, self.longitude as f64, 0.0)
        .distance_to(&GeoCoordinate::new(self.latitude as f64, (self.longitude + 1) as f64, 0.0))
        .unwrap() as usize
    )
  }
}