use crate::tile_storage::tile_signature::TileSignature;
use crate::tile_storage::tile_storage::STORAGE;

mod tile_storage;
mod errors;
mod config;
mod utils;

fn main()
{
  pretty_env_logger::init();
  let mut cfg = STORAGE.lock().unwrap();
  let x = cfg.get_or_emplace(TileSignature::from_f64(
    60.0, 30.0
  ));
  println!("{}", x.is_ok());
}