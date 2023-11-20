use std::fmt::Display;
use crate::errors::Error;
use crate::positioning::geocoordinate::GeoCoordinate;
use crate::positioning::utils::clip_latitude;

#[derive(Debug)]
pub enum ExtendMode
{
  Extend,
  Shrink
}

#[derive(Debug, Copy, Clone)]
pub struct GeoRectangle
{
  pub top_left: GeoCoordinate,
  pub bottom_right: GeoCoordinate
}

impl Display for GeoRectangle
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "[{}, {}]", self.top_left, self.bottom_right)
  }
}

impl GeoRectangle
{
  pub fn new(top_left: GeoCoordinate, bottom_right: GeoCoordinate) -> GeoRectangle
  {
    GeoRectangle
    {
      top_left,
      bottom_right
    }
  }

  pub fn from_tuples(top_left: (f64, f64), bottom_right: (f64, f64)) -> GeoRectangle
  {
    GeoRectangle
    {
      top_left: GeoCoordinate::new(top_left.0, top_left.1, 0.0),
      bottom_right: GeoCoordinate::new(bottom_right.0, bottom_right.1, 0.0)
    }
  }

  pub fn from_center_and_size(center: GeoCoordinate, width: f32, height: f32)
    -> Result<GeoRectangle, Error>
  {
    Ok(GeoRectangle::new(
      center
        .at_distance_and_azimuth(height / 2.0, 0.0, 0.0)?
        .at_distance_and_azimuth(width / 2.0, 270.0, 0.0)?,
      center
        .at_distance_and_azimuth(height / 2.0, 180.0, 0.0)?
        .at_distance_and_azimuth(width / 2.0, 90.0, 0.0)?
    ))
  }

  pub fn width_meters(&self) -> Result<f32, Error>
  {
    Ok(self.top_left.distance_to(&GeoCoordinate::new(
      self.top_left.latitude,
      self.bottom_right.longitude,
      0.0))?)
  }

  pub fn height_meters(&self) -> Result<f32, Error>
  {
    Ok(self.top_left.distance_to(&GeoCoordinate::new(
      self.bottom_right.latitude,
      self.top_left.longitude,
      0.0
    ))?)
  }

  pub fn size(&self) -> Result<(f32, f32), Error>
  {
    Ok((self.width_meters()?, self.height_meters()?))
  }

  pub fn center(&self) -> Result<GeoCoordinate, Error>
  {
    if self.top_left == self.bottom_right {
      return Err(Error::OperationOnInvalidCoordinate);
    }

    let c_lat = (self.top_left.latitude + self.bottom_right.latitude) / 2.0;
    let mut c_lon = (self.top_left.longitude + self.bottom_right.longitude) / 2.0;
    if self.top_left.longitude > self.bottom_right.longitude {
      c_lon -= 180.0;
    }
    Ok(GeoCoordinate::new(
      c_lat,
      clip_latitude(c_lon),
      0.0
    ))
  }

  pub fn to_square(&self, mode: ExtendMode) -> Result<GeoRectangle, Error>
  {
    let width = self.width_meters()?;
    let height = self.height_meters()?;
    let side = match mode {
      ExtendMode::Extend => width.max(height),
      ExtendMode::Shrink => width.min(height)
    };
    Ok(GeoRectangle::from_center_and_size(self.center()?, side, side)?)
  }
}