use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::{Div, Mul};
use std::path::MAIN_SEPARATOR;
use image::{GrayImage, ImageBuffer, Luma};
use indicatif::{ProgressBar, ProgressStyle};
use json::object;
use log::{debug, error, info};
use meridian_positioning::{CardinalDirection, GeoRectangle};
use num_derive::FromPrimitive;
use crate::elevation::elevation::Elevation;
use crate::errors::Error;
use crate::utils::replace_extension;

#[derive(Debug, PartialEq, FromPrimitive)]
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

#[derive(Debug, FromPrimitive)]
pub enum Resolution
{
  UltraLow,
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
      Resolution::UltraLow => 513,
      Resolution::Low => 1025,
      Resolution::Medium => 2049,
      Resolution::High => 4097
    }
  }
}

pub fn convert_georectangle(target_path: &str, georectangle: GeoRectangle,
                            target_size: Resolution, format: ImageFormat)
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
  info!("Target path:\t\t {}", path);

  // let square = match shape_mode {
  //   ShapeMode::Square => georectangle.to_square(ExtendMode::Extend)?,
  //   ShapeMode::AsProvided => georectangle
  // };
  let square = georectangle.clone();
  debug!("New georectangle: {}", square);
  debug!("Centers: old: {}, new: {}", georectangle.center(), square.center());
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
    let base_coordinate = square.top_left()
      .at_distance_and_azimuth(i as f32 * square.height_meters()? / size as f32,
                               CardinalDirection::South.to_degrees())?;
    for j in 0..size {
      let elevation = base_coordinate
        .at_distance_and_azimuth(j as f32 * square.width_meters()? / size as f32,
                                 CardinalDirection::East.to_degrees())?
        .elevation()
        .unwrap_or(0.0);
      min_max.0 = elevation.min(min_max.0 as f32) as i16;
      min_max.1 = elevation.max(min_max.1 as f32) as i16;
      table[i][j] = elevation as i16;
    }
  }
  pb1.finish_with_message(format!("Min/max found: {:?}", min_max));

  save_json_info(replace_extension(path.as_str(), "json").as_str(), min_max)?;

  let mut image: Box<GrayImage> = Box::new(
    ImageBuffer::new(size as u32, size as u32)
  );
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

  fs::create_dir_all(path[..path.rfind(MAIN_SEPARATOR).unwrap_or(path.len())]
    .to_string())?;

  debug!("Saving conversion result to {}...", &path);
  save_image(image.as_ref(), &path)
}

fn save_image(image: &ImageBuffer<Luma<u8>, Vec<u8>>, path: &str)
  -> Result<(), Error>
{
  image.save(path)?;
  info!("Image saved to {}", &path);
  Ok(())
}

fn save_json_info(path: &str, min_max: (i16, i16)) -> Result<(), Error>
{
  let json = object!
  {
    heightmap:
    {
      min: min_max.0,
      max: min_max.1
    }
  };

  let mut file = File::create(path)?;
  file.write_all(json
    .pretty(4)
    .as_bytes())?;
  Ok(())
}