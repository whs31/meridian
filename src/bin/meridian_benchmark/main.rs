use std::env;
use meridian_positioning::positioning::{GeoCoordinate, GeoRectangle};

use meridian::heightmap::{convert_georectangle, ImageFormat, Resolution};
use meridian::init_logger;

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
  let rectangle = GeoRectangle::from_center_meters(
    GeoCoordinate::new(45.285843, 34.238057, None),
    300_000.0,
    300_000.0
  ).expect("Failed to create GeoRectangle");
  let _ = match convert_georectangle(path.as_str(),
                               rectangle,
                               Resolution::High,
                               ImageFormat::PNG
  )
  {
    Ok(_) => (),
    Err(e) => {
      eprintln!("{}", e);
    }
  };
  println!("Done!");
}