use log::{debug, info};
use crate::errors::Error;
use crate::positioning::georectangle::{ExtendMode, GeoRectangle};

pub enum ImageFormat
{
  PNG
}

pub fn convert_georectangle(target_path: &str, georectangle: GeoRectangle, format: ImageFormat)
  -> Result<(), Error>
{
  info!("Converting georectangle {:?}", &georectangle);
  debug!("Georectangle size: {:?}", georectangle.size());
  debug!("New georectangle: {:?}", georectangle.to_square(ExtendMode::Extend)?);
  debug!("New georectangle size: {:?}", georectangle.to_square(ExtendMode::Extend)?.size());
  Ok(())
}

#[cfg(test)]
mod tests
{
  use crate::init_logger;
  use crate::positioning::geocoordinate::GeoCoordinate;
  use super::*;

  #[test]
  fn test_convert_georectangle()
  {
    init_logger();
    let _ = convert_georectangle("123", GeoRectangle::new(
      GeoCoordinate::new(61.0, 31.0, 0.0),
      GeoCoordinate::new(60.0, 30.0, 0.0),
    ), ImageFormat::PNG);
  }
}