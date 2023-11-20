use crate::{elevation, init_logger};
use once_cell::sync::Lazy;
use std::env;
use std::ffi::{c_char, c_double, c_int, CString};

static BINARY_DIRECTORY: Lazy<String> = Lazy::new(|| {
  env::current_dir()
    .unwrap()
    .into_os_string()
    .into_string()
    .unwrap()
});

#[repr(C)]
pub struct MeridianVersion
{
  pub major: c_int,
  pub minor: c_int,
  pub patch: c_int
}

#[no_mangle]
#[allow(dead_code)]
pub extern fn meridian_version() -> MeridianVersion
{
  MeridianVersion {
    major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
    minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
    patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap()
  }
}

#[no_mangle]
#[allow(dead_code)]
pub extern fn meridian_binary_directory() -> *const c_char
{
  CString::new(BINARY_DIRECTORY.clone()).unwrap().into_raw() as *const _
}

#[no_mangle]
#[allow(dead_code)]
pub extern fn meridian_elevation(latitude: c_double, longitude: c_double) -> c_int
{
  match elevation::elevation::elevation_at((latitude, longitude)) {
    Ok(value) => value as c_int,
    Err(_) => -404
  }
}

#[no_mangle]
#[allow(dead_code)]
pub extern fn meridian_enable_logger() -> bool
{
  init_logger()
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_init_logger()
  {
    meridian_enable_logger();
  }
  #[test]
  fn test_elevation_at()
  {
    assert_eq!(meridian_elevation(0.0, 0.0), -404);
    assert_eq!(meridian_elevation(60.0, 30.0), 0);
    assert_eq!(meridian_elevation(61.0, 31.0), 3);
    assert_eq!(meridian_elevation(60.9, 30.9), 3);
    assert_eq!(meridian_elevation(60.5, 30.5), 62);
  }
}