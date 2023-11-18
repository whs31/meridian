use log::warn;
pub use crate::elevation::elevation::elevation_at;

mod tile_storage;
pub mod errors;
pub mod config;
mod utils;
pub mod elevation;
pub mod ffi;
pub mod positioning;

pub fn init_logger() -> bool
{
  return match pretty_env_logger::try_init() {
    Ok(_) => true,
    Err(_) => {
      warn!("Failed to initialize logger");
      false
    }
  }
}

#[cfg(test)]
mod tests
{
  use crate::{elevation_at, init_logger};

  #[test]
  fn test_elevation_at()
  {
    assert!(elevation_at((0.0, 0.0)).is_err());
    assert!(elevation_at((60.0, 30.0)).unwrap().abs() - 0.0 < 1.0);
    assert!(elevation_at((61.0, 31.0)).unwrap().abs() - 3.0 < 1.0);
  }
}
