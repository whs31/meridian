use float_cmp::approx_eq;
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

#[derive(Debug, Copy, Clone)]
pub struct GeoCoordinate
{
  pub latitude: f64,
  pub longitude: f64,
  pub altitude: f64
}

impl PartialEq for GeoCoordinate
{
  fn eq(&self, other: &GeoCoordinate) -> bool
  {
    approx_eq!(f64, self.latitude, other.latitude, epsilon = 0.0000003) &&
    approx_eq!(f64, self.longitude, other.longitude, epsilon = 0.0000003) &&
    approx_eq!(f64, self.altitude, other.altitude, epsilon = 0.0000003)
  }
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

  pub fn new_2d(latitude: f64, longitude: f64) -> GeoCoordinate
  {
    GeoCoordinate
    {
      latitude,
      longitude,
      altitude: 0.0
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
    let az = azimuth as f64;
    let ratio = (distance / consts::EARTH_MEAN_RADIUS) as f64;
    let r_lat = (self.latitude
      .to_radians()
      .sin() * ratio.cos() + self.latitude
      .to_radians()
      .cos() * ratio.sin() * az
      .to_radians()
      .cos()).asin()
      .to_degrees();
    let r_lon = (self.longitude.to_radians() + (az
      .to_radians()
      .sin() * ratio.sin() * self.latitude
      .to_radians()
      .cos()).atan2((ratio.cos() - self.latitude
      .to_radians()
      .sin() * r_lat.sin())))
      .to_degrees();
    Ok(GeoCoordinate::new(r_lat,
                          clip_longitude(r_lon),
                          self.altitude + up as f64))
  }
}

#[cfg(test)]
mod tests
{
  use std::collections::VecDeque;
  use crate::init_logger;
  use super::*;

  #[test]
  fn test_at_distance_and_azimuth()
  {
    init_logger();

    let t_coord = GeoCoordinate::new_2d(60.0, 30.0);
    let d: Vec<f32> = vec![10000.0, -10000.0, 55600.0, -43400.0];
    let az: Vec<f32> = vec![0.0, 90.0, 180.0, 270.0, 360.0];
    let mut expected: VecDeque<GeoCoordinate> = VecDeque::from(vec![
      GeoCoordinate::new_2d(60.089932059, 30.0),
      GeoCoordinate::new_2d(59.999877754, 30.179863675),
      GeoCoordinate::new_2d(59.910067941, 30.0),
      GeoCoordinate::new_2d(59.999877754, 29.820136325),
      GeoCoordinate::new_2d(60.089932059, 30.0),
      GeoCoordinate::new_2d(59.910067941, 30.0),
      GeoCoordinate::new_2d(59.999877754, 29.820136325),
      GeoCoordinate::new_2d(59.999877754, 30.179863675),
      GeoCoordinate::new_2d(59.910067941, 30.0),
      GeoCoordinate::new_2d(60.500022248, 30.0),
      GeoCoordinate::new_2d(59.996221155, 30.999968343),
      GeoCoordinate::new_2d(59.499977752, 30.0),
      GeoCoordinate::new_2d(59.996221155, 29.000031657),
      GeoCoordinate::new_2d(60.500022248, 30.0),
      GeoCoordinate::new_2d(59.609694864, 30.0),
      GeoCoordinate::new_2d(59.997697499, 29.219425949),
      GeoCoordinate::new_2d(60.390305136, 30.0),
      GeoCoordinate::new_2d(59.997697499, 30.780574051),
      GeoCoordinate::new_2d(59.609694864, 30.0),
    ]);

    for distance in d.iter() {
      for azimuth in az.iter() {
        println!("Testing at_distance_and_azimuth({}, {}, 0.0)", distance, azimuth);
        assert_eq!(t_coord.at_distance_and_azimuth(*distance, *azimuth, 0.0).unwrap(),
                   expected.pop_front().unwrap());
      }
    }
  }
}