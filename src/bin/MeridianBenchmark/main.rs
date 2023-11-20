use std::env;
use log::info;

use meridian::heightmap::heightmap_conversion::{convert_georectangle, ImageFormat};
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
  let rectangle = GeoRectangle::from_tuples((60.548257, 27.814114),
                                            (56.955401, 38.238276));
  let _ = convert_georectangle(path.as_str(),
                               rectangle,
                               2048,
                               (0.0, 200.0),
                               ImageFormat::PNG);
  info!("Done!");
}