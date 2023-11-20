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
  let rectangle = GeoRectangle::from_tuples((47.331179, 37.645298),
                                            (46.451195, 39.292670));
  let _ = convert_georectangle(path.as_str(),
                               rectangle,
                               2048,
                               (0.0, 200.0),
                               ImageFormat::PNG);
  info!("Done!");
}