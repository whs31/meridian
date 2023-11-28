use std::env;
use meridian_positioning::{GeoCoordinate, GeoRectangle};
use meridian::Chunk;
use meridian::config::CONFIG;
use meridian::heightmap::ElevationPrefetcher;
// use meridian::heightmap::{convert_georectangle, ImageFormat, Resolution};
use meridian::init_logger;

fn main()
{
  init_logger();
  println!("Starting MeridianBenchmark...");

  let rectangle = GeoRectangle::from_center_meters(
    GeoCoordinate::new(45.285843, 34.238057, None),
    300_000.0,
    300_000.0
  ).expect("Failed to create GeoRectangle");

  let cfg = CONFIG.lock().unwrap();
  let mut p = ElevationPrefetcher::new(
    cfg.get("Elevation", "remote_url").unwrap(),
    cfg.get("Elevation", "cache_dir").unwrap(),
    cfg.get("Elevation", "extension").unwrap(),
    cfg.get("Elevation", "max_parallel_threads").unwrap().parse().unwrap()
  );
  p.fetch(rectangle);
  let path = env::current_dir()
    .unwrap()
    .join("test-chunk")
    .into_os_string()
    .into_string()
    .unwrap();

  // let _ = match convert_georectangle(path.as_str(),
  //                              rectangle,
  //                              Resolution::High,
  //                              ImageFormat::PNG
  // )
  // {
  //   Ok(_) => (),
  //   Err(e) => {
  //     eprintln!("{}", e);
  //   }
  // };
  // let chunk = Chunk::new(
  //   GeoCoordinate::new(45.285843, 34.238057, None),
  //   30_000,
  //   18,
  //   path.as_str()
  // ).expect("Failed to create Chunk");
  // println!("{}", chunk);
  // println!("{:?}", chunk);
  println!("Done!");
}