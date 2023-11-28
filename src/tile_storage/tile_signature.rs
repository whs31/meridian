use std::fmt::Display;
use std::path::MAIN_SEPARATOR;
use meridian_positioning::GeoCoordinate;
use crate::config::CONFIG;
use crate::tile_storage::Quarter;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
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

  pub fn georectangle_size(&self) -> (usize, usize)
  {
    (
      GeoCoordinate::new(self.latitude as f64, self.longitude as f64, None)
        .distance_to(&GeoCoordinate::new((self.latitude + 1) as f64, self.longitude as f64, None))
        .unwrap() as usize,
      GeoCoordinate::new(self.latitude as f64, self.longitude as f64, None)
        .distance_to(&GeoCoordinate::new(self.latitude as f64, (self.longitude + 1) as f64, None))
        .unwrap() as usize
    )
  }

  pub fn to_relative_path(&self, extension: &str) -> String
  {
    let dot = if extension.is_empty() { "" } else { "." };
    return format!("{}{MAIN_SEPARATOR}{}{MAIN_SEPARATOR}{}{dot}{extension}",
                   self.quarter().to_u8(),
                   self.latitude.abs(),
                   self.longitude.abs()
    ).to_string()
  }

  pub fn to_abs_path(&self) -> String
  {
    let cfg = CONFIG.lock().unwrap();
    let extension = cfg
      .get("Elevation", "extension")
      .unwrap_or("tif".to_string());
    self.to_abs_path_threadsafe(extension.as_str(), cfg
      .get("Elevation", "cache_dir")
      .unwrap().as_str()
    )
  }

  pub fn to_abs_path_threadsafe(&self, extension: &str, cache_dir: &str) -> String
  {
    format!("{}{MAIN_SEPARATOR}{}{MAIN_SEPARATOR}{}",
            std::env::current_dir()
              .unwrap()
              .into_os_string()
              .into_string()
              .unwrap(),
            cache_dir, self.to_relative_path(extension)
    )
  }

  pub fn to_url(&self) -> String
  {
    let cfg = CONFIG.lock().unwrap();
    let extension = cfg
      .get("Elevation", "extension")
      .unwrap_or("tif".to_string());

    self.to_url_threadsafe(extension.as_str(),
                           cfg.get("Elevation", "remote_url").unwrap().as_str()
    )
  }

  pub fn to_url_threadsafe(&self, extension: &str, url: &str) -> String
  {
    format!("{}/{}", url, self.to_relative_path(extension))
      .replace('\\', "/")
  }
}