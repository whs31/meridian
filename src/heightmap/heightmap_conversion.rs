use std::fs;
use std::ops::{Div, Mul};
use std::path::MAIN_SEPARATOR;
use image::{GrayImage, ImageBuffer, Luma};
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error, info};
use crate::elevation::elevation::Elevation;
use crate::errors::Error;
use crate::positioning::georectangle::{ExtendMode, GeoRectangle};

#[derive(Debug, PartialEq)]
pub enum ImageFormat
{
  PNG,
  RAW
}

impl ImageFormat
{
  pub fn extension(&self) -> &str
  {
    match self
    {
      ImageFormat::PNG => "png",
      ImageFormat::RAW => "raw"
    }
  }

  pub fn is_8bit(&self) -> bool
  {
    match self
    {
      ImageFormat::PNG => true,
      ImageFormat::RAW => false
    }
  }
}

#[derive(Debug)]
pub enum ShapeMode
{
  Square,
  AsProvided
}

#[derive(Debug)]
pub enum Resolution
{
  Low,
  Medium,
  High
}

impl Resolution
{
  pub fn value(&self) -> usize
  {
    match self
    {
      Resolution::Low => 1024,
      Resolution::Medium => 2048,
      Resolution::High => 4096
    }
  }
}

pub fn convert_georectangle(target_path: &str, georectangle: GeoRectangle,
                            target_size: Resolution, bounds: (f32, f32),
                            format: ImageFormat, shape_mode: ShapeMode)
  -> Result<(), Error>
{
  if format == ImageFormat::RAW {
    error!("Not implemented for RAW format");
    return Err(Error::NotImplemented);
  }

  let size = target_size.value();
  let path = format!("{target_path}.{}", format.extension());
  info!("Converting georectangle {}", &georectangle);
  info!("Target size:\t\t {}x{} px", size, size);
  info!("Format:\t\t {:?} (.{})", format, format.extension());
  info!("Bounds:\t\t {:?}", bounds);
  info!("Target path:\t\t {}", path);

  debug!("Georectangle size: {:?}", georectangle.size());
  let square = match shape_mode {
    ShapeMode::Square => georectangle.to_square(ExtendMode::Extend)?,
    ShapeMode::AsProvided => georectangle
  };
  debug!("New georectangle: {}", square);
  debug!("New georectangle size: {:?}", square.size());
  debug!("Centers: old: {}, new: {}", georectangle.center()?, square.center()?);

  debug!("Finding min/max...");
  let pb1 = ProgressBar::new(size as u64);
  pb1.set_style(ProgressStyle::with_template(
    "{wide_msg} {spinner:.green} [{bar:20.red/orange}] \
    {human_pos:10}/ {human_len:10} ({percent:3}%)",)
    .unwrap()
    .progress_chars("█░░"));
  pb1.set_message(format!("Finding min/max"));
  let mut min_max = (i16::MAX, i16::MIN);
  let mut table: Vec<Vec<i16>> = vec![vec![0; size]; size];
  for i in 0..size {
    pb1.set_position(i as u64);
    let base_coordinate = square.top_left
      .at_distance_and_azimuth(i as f32 * square.height_meters()? / size as f32,
                               180.0, 0.0)?;
    for j in 0..size {
      let elevation = base_coordinate
        .at_distance_and_azimuth(j as f32 * square.width_meters()? / size as f32,
                                 90.0, 0.0)?
        .elevation()
        .unwrap_or(0.0);
      min_max.0 = elevation.min(min_max.0 as f32) as i16;
      min_max.1 = elevation.max(min_max.1 as f32) as i16;
      table[i][j] = elevation as i16;
    }
  }
  pb1.finish_with_message(format!("Min/max found: {:?}", min_max));
  debug!("Min/max: {:?}", min_max);

  let mut image: GrayImage = ImageBuffer::new(size as u32, size as u32);
  debug!("Converting...");
  let pb = ProgressBar::new(size as u64);
  pb.set_style(ProgressStyle::with_template(
    "{wide_msg} {spinner:.green} [{bar:20.cyan/blue}] \
    {human_pos:10}/ {human_len:10} ({percent:3}%)",)
    .unwrap()
    .progress_chars("█░░"));
  pb.set_message(format!("Converting to {}x{} px...", size, size));

  let clamp = match format {
    ImageFormat::PNG => u8::MAX as f32,
    ImageFormat::RAW => u16::MAX as f32
  };

  // sanity warning!
  table
    .iter()
    .enumerate()
    .for_each(|(i, row)| {
      pb.set_position(i as u64);
      let new_row: Vec<i16> = row
        .iter()
        .map(|&pixel| {
          let elevation = (pixel as f32 - min_max.0 as f32)
            .div(min_max.1 as f32 - min_max.0 as f32)
            .mul(u8::MAX as f32)
            .clamp(0.0, clamp) as i16;
          elevation
      }).collect();
    new_row
      .iter()
      .enumerate()
      .for_each(|(j, &elevation)| {
        image.put_pixel(j as u32, i as u32, Luma([elevation as u8]));
      });
  });

  pb.finish_with_message(" Conversion done!");
  debug!("Making missing folders to target {target_path}...");

  fs::create_dir_all(path[..path.rfind(MAIN_SEPARATOR).unwrap()]
    .to_string())
    .unwrap();

  debug!("Saving conversion result to {}...", &path);
  save_image(&image, &path)
}

fn save_image(image: &ImageBuffer<Luma<u8>, Vec<u8>>, path: &str)
  -> Result<(), Error>
{
  match image.save(path) {
    Ok(_) => {
      info!("Image saved to {}", &path);
      Ok(())
    },
    Err(e) => Err(Error::ImageSaveFailure(e))
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
      .join("test-convert_georectangle")
      .into_os_string()
      .into_string()
      .unwrap();
    let rectangle = GeoRectangle::from_tuples((61.0, 30.0), (60.0, 31.0));
    let _ = convert_georectangle(path.as_str(),
                                 rectangle,
                                 Resolution::Low,
                                 (0.0, 200.0),
                                 ImageFormat::PNG,
                                 ShapeMode::Square);
  }
}