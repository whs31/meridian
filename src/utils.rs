use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::errors::Error;
use crate::positioning::geocoordinate::GeoCoordinate;

pub type StaticHeapObject<T> = Lazy<Mutex<Box<T>>>;

pub fn validate_coordinate(coordinate: (f64, f64)) -> Result<(f64, f64), Error>
{
  if coordinate.0 < -90.0 || coordinate.0 > 90.0 || coordinate.1 < -180.0 || coordinate.1 > 180.0 {
    return Err(Error::OperationOnInvalidCoordinate(GeoCoordinate::new_2d(coordinate.0, coordinate.1)));
  }
  const THRESHOLD: f64 = 0.00001;
  let mut lat = coordinate.0;
  let mut lon = coordinate.1;
  if coordinate.0 - coordinate.0.floor() < THRESHOLD { lat = coordinate.0.floor(); }
  if coordinate.0.ceil() - coordinate.0 < THRESHOLD { lat = coordinate.0.ceil(); }
  if coordinate.1 - coordinate.1.floor() < THRESHOLD { lon = coordinate.1.floor(); }
  if coordinate.1.ceil() - coordinate.1 < THRESHOLD { lon = coordinate.1.ceil(); }

  return Ok((lat, lon));
}

pub fn replace_extension(path: &str, new_extension: &str) -> String
{
  let mut p = path.to_string();
  let i = p.rfind('.').unwrap();
  p.replace_range((i + 1).., new_extension);
  return p;
}