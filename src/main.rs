use crate::elevation::elevation::elevation_at;
use crate::tile_storage::tile_storage::STORAGE;

mod tile_storage;
mod errors;
mod config;
mod utils;
mod elevation;

fn main()
{
  pretty_env_logger::init();
  let x = elevation_at((0.0, 0.0));
  let y = elevation_at((60.0, 30.0));
  let z = elevation_at((61.0, 31.0));
  println!("{} {} {}", x.unwrap_or(-404.0), y.unwrap_or(-404.0), z.unwrap_or(-404.0));
}