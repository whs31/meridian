use std::env;

use meridian::heightmap::heightmap_conversion::{convert_georectangle, ImageFormat, Resolution, ShapeMode};
use meridian::init_logger;
use meridian::positioning::geocoordinate::GeoCoordinate;
use meridian::positioning::georectangle::GeoRectangle;

fn main()
{
  init_logger();
  println!("Starting MeridianBenchmark...");
  let path = env::current_dir()
    .unwrap()
    .join("test-result")
    .join("test-convert")
    .into_os_string()
    .into_string()
    .unwrap();
  let rectangle = GeoRectangle::from_center_and_size(
    GeoCoordinate::new_2d(45.0, 80.0),
    5_000_000.0,
    5_000_000.0
  ).expect("Failed to create GeoRectangle");
  let _ = match convert_georectangle(path.as_str(),
                               rectangle,
                               Resolution::Low,
                               (0.0, 200.0),
                               ImageFormat::PNG,
                               ShapeMode::AsProvided)
  {
    Ok(_) => (),
    Err(e) => {
      eprintln!("{}", e);
    }
  };
  println!("Done!");
}