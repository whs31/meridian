use std::env;
use log::info;

use meridian::heightmap::heightmap_conversion::{convert_georectangle, ImageFormat, ShapeMode};
use meridian::init_logger;
use meridian::positioning::georectangle::GeoRectangle;

fn main()
{
  init_logger();
  info!("Starting MeridianBenchmark...");
  let path = env::current_dir()
    .unwrap()
    .join("test-result")
    .join("test-convert_georectangle.png")
    .into_os_string()
    .into_string()
    .unwrap();
  let rectangle = GeoRectangle::from_tuples((60.0, 30.0),
                                            (58.0, 32.0));
  let _ = convert_georectangle(path.as_str(),
                               rectangle,
                               4096,
                               (0.0, 200.0),
                               ImageFormat::PNG,
                               ShapeMode::AsProvided);
  info!("Done!");
}