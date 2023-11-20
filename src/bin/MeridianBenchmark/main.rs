use std::env;
use log::info;

use meridian::heightmap::heightmap_conversion::{convert_georectangle, ImageFormat, Resolution, ShapeMode};
use meridian::init_logger;
use meridian::positioning::geocoordinate::GeoCoordinate;
use meridian::positioning::georectangle::GeoRectangle;

fn main()
{
  init_logger();
  info!("Starting MeridianBenchmark...");
  let path = env::current_dir()
    .unwrap()
    .join("test-result")
    .join("test-convert")
    .into_os_string()
    .into_string()
    .unwrap();
  let rectangle = GeoRectangle::from_center_and_size(
    GeoCoordinate::new_2d(55.75431502026738, 37.61903345376926),
    100000.0,
    100000.0
  ).expect("Failed to create GeoRectangle");
  let _ = convert_georectangle(path.as_str(),
                               rectangle,
                               Resolution::Low,
                               (0.0, 200.0),
                               ImageFormat::RAW,
                               ShapeMode::Square);
  info!("Done!");
}