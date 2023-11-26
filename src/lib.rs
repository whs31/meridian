use log::warn;
pub use crate::elevation::elevation::elevation_at;

mod geotiff;
mod tile_storage;
pub mod errors;
pub mod config;
mod utils;
pub mod elevation;
mod ffi;
pub mod heightmap;
mod tile_map;
mod coordinate_system;
pub use coordinate_system::Chunk;

pub fn init_logger() -> bool
{
  return match pretty_env_logger::try_init() {
    Ok(_) => true,
    Err(_) => {
      warn!("Failed to initialize logger: Logger is already initialized");
      false
    }
  }
}
