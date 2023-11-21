use std::fmt::Display;
use float_cmp::approx_eq;
use crate::elevation::elevation::{Elevation, elevation_at_coordinate};
use crate::errors::Error;
use crate::positioning::consts;
use crate::positioning::utils::{clip_longitude, is_valid_latitude, is_valid_longitude};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GeoCoordinateType
{
  InvalidCoordinate,
  Coordinate2D,
  Coordinate3D
}

#[derive(Copy, Clone, Debug)]
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

impl Display for GeoCoordinate
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "({:.7}°, {:.7}°, {:.1} m)", self.latitude, self.longitude, self.altitude)
  }
}

impl Elevation for GeoCoordinate
{
  fn elevation(&self) -> Result<f32, Error>
  {
    elevation_at_coordinate(self)
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
    if is_valid_latitude(self.latitude) && is_valid_longitude(self.longitude) {
      return if approx_eq!(f64, self.altitude, 0.0) {
        GeoCoordinateType::Coordinate2D
      } else {
        GeoCoordinateType::Coordinate3D
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
      return Err(Error::OperationOnInvalidCoordinatePair(self.clone(), other.clone()));
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
      return Err(Error::OperationOnInvalidCoordinatePair(self.clone(), other.clone()));
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
      return Err(Error::OperationOnInvalidCoordinate(self.clone()));
    }

    let ratio = distance as f64 / consts::EARTH_MEAN_RADIUS as f64;
    let lat = (self.latitude
      .to_radians()
      .sin() * ratio.cos() + self.latitude
      .to_radians()
      .cos() * ratio.sin() * (azimuth as f64)
      .to_radians()
      .cos())
      .asin();
    Ok(GeoCoordinate::new(
      lat.to_degrees(),
      clip_longitude((self.longitude
        .to_radians() + ((azimuth as f64)
        .to_radians()
        .sin() *
        ratio.sin() * self.latitude
        .to_radians()
        .cos())
        .atan2(ratio.cos() - self.latitude
          .to_radians()
          .sin() * lat.sin()))
        .to_degrees()),
      self.altitude + up as f64
    ))
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
      GeoCoordinate::new_2d(60.089932059, 30.0),
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
    ]);

    for distance in d.iter() {
      for azimuth in az.iter() {
        println!("Testing at_distance_and_azimuth({}, {}, 0.0)", distance, azimuth);
        assert_eq!(t_coord.at_distance_and_azimuth(*distance, *azimuth, 0.0).unwrap(),
                   expected.pop_front().unwrap_or(t_coord.at_distance_and_azimuth(*distance,
                                                                                  *azimuth, 0.0)
                     .unwrap()));
        println!("Result: OK");
      }
    }
  }
}