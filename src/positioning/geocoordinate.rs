use crate::errors::Error;
use crate::positioning::consts;
use crate::positioning::utils::{clip_longitude, is_valid_latitude};

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum GeoCoordinateType
{
  InvalidCoordinate,
  Coordinate2D,
  Coordinate3D
}

#[derive(Debug)]
pub struct GeoCoordinate
{
  pub latitude: f64,
  pub longitude: f64,
  pub altitude: f64
}

impl GeoCoordinate
{
  pub fn new(latitude: f64, longitude: f64, altitude: f64) -> GeoCoordinate
  {
    GeoCoordinate
    {
      latitude,
      longitude,
      altitude
    }
  }

  pub fn coordinate_type(&self) -> GeoCoordinateType
  {
    if is_valid_latitude(self.latitude) && is_valid_latitude(self.longitude) {
      return match self.altitude {
        0.0 => GeoCoordinateType::Coordinate2D,
        _ => GeoCoordinateType::Coordinate3D
      }
    }
    GeoCoordinateType::InvalidCoordinate
  }

  pub fn valid(&self) -> bool
  {
    self.coordinate_type() != GeoCoordinateType::InvalidCoordinate
  }

  pub fn azimuth_to(&self, other: &GeoCoordinate) -> Result<f32, Error>
  {
    if !self.valid() || !other.valid() {
      return Err(Error::OperationOnInvalidCoordinate);
    }
    let d_lon = (other.longitude - self.longitude).to_radians();
    let lat1_rad = self.latitude.to_radians();
    let lat2_rad = other.latitude.to_radians();
    let y = d_lon.sin() * lat2_rad.cos();
    let x = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * d_lon.cos();
    let azimuth = y.atan2(x).to_degrees() + 360.0;
    let whole = azimuth.trunc();
    let fraction = azimuth.fract();
    Ok(((whole + 360.0) as i32 % 360) as f32 + fraction as f32)
  }

  pub fn distance_to(&self, other: &GeoCoordinate) -> Result<f32, Error>
  {
    if !self.valid() || !other.valid() {
      return Err(Error::OperationOnInvalidCoordinate);
    }
    let d_lat = (other.latitude - self.latitude).to_radians();
    let d_lon = (other.longitude - self.longitude).to_radians();
    let haversine_d_lat = (d_lat / 2.0).sin().powi(2);
    let haversine_d_lon = (d_lon / 2.0).sin().powi(2);
    let y = haversine_d_lat + self.latitude.to_radians().cos()
      * other.latitude.to_radians().cos()
      * haversine_d_lon;
    let x = 2.0 * y.sqrt().asin();
    Ok(consts::EARTH_MEAN_RADIUS * x as f32)
  }

  pub fn at_distance_and_azimuth(&self, distance: f32, azimuth: f32, up: f32)
    -> Result<GeoCoordinate, Error>
  {
    if !self.valid() {
      return Err(Error::OperationOnInvalidCoordinate);
    }
    let ratio = distance as f64 / consts::EARTH_MEAN_RADIUS as f64;
    let result_lat_rad = (self.latitude.to_radians().sin() * ratio.cos()
      + self.latitude.to_radians().cos() * ratio.sin()
      * (azimuth as f64).to_radians().cos()).asin();
    let result_lon_rad = self.longitude.to_radians()
      + ((azimuth as f64).to_radians().sin() * ratio.sin() * self.latitude.to_radians().cos())
      .atan2(ratio.cos() - self.latitude.to_radians().sin() * result_lat_rad.sin());
    return Ok(GeoCoordinate::new(result_lat_rad.to_degrees(),
                                 clip_longitude(result_lon_rad.to_degrees()),
                                 self.altitude + up as f64));
  }
}