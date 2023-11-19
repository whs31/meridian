use crate::errors::Error;
use crate::positioning::geocoordinate::GeoCoordinate;

pub fn is_valid_latitude(lat: f64) -> bool
{
  lat >= -90.0 && lat <= 90.0
}

pub fn is_valid_longitude(lon: f64) -> bool
{
  lon >= -180.0 && lon <= 180.0
}

pub fn clip_latitude(lat: f64) -> f64
{
  if lat > 90.0 { return 90.0 }
  else if lat < -90.0 { return -90.0 }
  lat
}

pub fn clip_longitude(lon: f64) -> f64
{
  if lon > 180.0 { return lon - 360.0}
  else if lon < -180.0 { return lon + 360.0 }
  lon
}

pub fn geopath_length(path: Vec<GeoCoordinate>, from: usize, mut to: usize) -> Result<f32, Error>
{
  if path.is_empty() {
    return Err(Error::EmptyPath);
  }
  let wrap = to == -1;
  if to < 0 || to >= path.len() {
    to = path.len() - 1;
  }
  let mut len = 0.0_f32;
  for i in from..to {
    len += path[i].to_wgs84().distance(&path[i + 1].to_wgs84());
  }
  if wrap {
    len += path.last().unwrap().to_wgs84().distance(&path.first().unwrap().to_wgs84());
  }
  Ok(len)
}