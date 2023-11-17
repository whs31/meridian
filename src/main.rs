use log::warn;

mod tile_storage;
mod errors;
mod config;
mod utils;

fn main()
{
  pretty_env_logger::init();
  warn!("{}", config::CONFIG.lock().unwrap().get("Elevation", "remote_url").unwrap());
  println!("Hello, world!");
}