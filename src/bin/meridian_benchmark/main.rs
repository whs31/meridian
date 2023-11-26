use std::env;
use meridian_positioning::positioning::{GeoCoordinate, GeoRectangle};

use meridian::heightmap::heightmap_conversion::{convert_georectangle, ImageFormat, Resolution};
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
    GeoCoordinate::new(58.92206844421908, 22.378208157377035, None),
    300_000.0,
    300_000.0
  ).expect("Failed to create GeoRectangle");
  let _ = match convert_georectangle(path.as_str(),
                               rectangle,
                               Resolution::Low,
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