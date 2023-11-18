use nav_types::WGS84;
use crate::errors::Error;
use crate::positioning::utils::is_valid_latitude;

#[derive(Debug)]
#[derive(PartialEq)]
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

  pub fn to_wgs84(&self) -> WGS84<f64>
  {
    WGS84::from_degrees_and_meters(self.latitude, self.longitude, self.altitude)
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
}