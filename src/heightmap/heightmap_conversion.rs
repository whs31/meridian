use std::fs;
use std::path::MAIN_SEPARATOR;
use image::{ImageBuffer, Rgb, RgbImage};
use log::{debug, error, info};
use crate::elevation::elevation::Elevation;
use crate::errors::Error;
use crate::positioning::georectangle::{ExtendMode, GeoRectangle};

#[derive(Debug)]
pub enum ImageFormat
{
  PNG
}

pub fn convert_georectangle(target_path: &str, georectangle: GeoRectangle,
                            target_size: usize, bounds: (f32, f32), format: ImageFormat)
  -> Result<(), Error>
{
  info!("Converting georectangle {}", &georectangle);
  info!("\t\tTarget size:\t\t {}x{} px", target_size, target_size);
  info!("\t\tFormat:\t\t\t\t\t {:?}", format);
  info!("\t\tBounds:\t\t\t\t\t {:?}", bounds);
  info!("\t\tTarget path:\t\t {}", target_path);

  debug!("Georectangle size: {:?}", georectangle.size());
  let square = georectangle.to_square(ExtendMode::Extend)?;
  debug!("New georectangle: {}", square);
  debug!("New georectangle size: {:?}", square.size());
  debug!("Centers: old: {}, new: {}", georectangle.center()?, square.center()?);

  let mut image: RgbImage = ImageBuffer::new(target_size as u32, target_size as u32);
  debug!("Converting...");
  for i in 0..target_size {
    let base_coordinate = square.top_left
      .at_distance_and_azimuth((i as f32 * square.height_meters()? / target_size as f32) as f32,
                               180.0, 0.0)?;
    for j in 0..target_size {
      let elevation = base_coordinate
        .at_distance_and_azimuth((j as f32 * square.width_meters()? / target_size as f32) as f32,
                                  90.0, 0.0)?
        .elevation()?
        .clamp(bounds.0, bounds.1)
        .clamp(u8::MIN as f32, u8::MAX as f32);
      let pixel = Rgb([elevation as u8, elevation as u8, elevation as u8]);
      image.put_pixel(i as u32, j as u32, pixel);
    }
  }
  info!("Converted!");
  debug!("Making missing folders to target {target_path}...");
  fs::create_dir_all(target_path[..target_path.rfind(MAIN_SEPARATOR).unwrap()].to_string())
    .unwrap();
  debug!("Saving conversion result to {}...", target_path);
  match image.save(target_path) {
    Ok(_) => {
      info!("Image saved to {}", target_path);
      Ok(())
    },
    Err(e) => {
      error!("Failed to save image: {}", e);
      Err(Error::ImageSaveFailure)
    }
  }
}

#[cfg(test)]
mod tests
{
  use std::env;
  use crate::init_logger;
  use super::*;

  #[test]
  fn test_convert_georectangle()
  {
    init_logger();
    let path = env::current_dir()
      .unwrap()
      .join("test-result")
      .join("test-convert_georectangle.png")
      .into_os_string()
      .into_string()
      .unwrap();
    let rectangle = GeoRectangle::from_tuples((61.0, 30.0), (60.0, 31.0));
    let _ = convert_georectangle(path.as_str(),
                                 rectangle,
                                 2048,
                                 (0.0, 200.0),
                                 ImageFormat::PNG);
  }
}