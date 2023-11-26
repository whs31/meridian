use std::path::MAIN_SEPARATOR;
use std::sync::Mutex;
use ini::configparser::ini::Ini;
use log::info;
use once_cell::sync::Lazy;
use crate::errors::Error;
use crate::utils::StaticHeapObject;

pub static CONFIG: StaticHeapObject<Config> = Lazy::new(
  || { Mutex::new(Box::new(Config::new())) }
);

static DEFAULT_FILENAME: &'static str = "cfg_meridian.ini";
static ELEVATION_SECTION: &'static str = "Elevation";

pub struct Config
{
  pub filename: String,
  ini: Ini
}

impl Config
{
  pub fn new() -> Self
  {
    let mut config = Ini::new();
    return match config.load(DEFAULT_FILENAME)
    {
      Ok(_) => {
        info!("Loading existing config file: {DEFAULT_FILENAME}");
        Self {
          filename: DEFAULT_FILENAME.to_string(),
          ini: config
        }
      }
      Err(_) => {
        info!("Created new config file: {DEFAULT_FILENAME}");
        config.set(ELEVATION_SECTION, "remote_url",
                   Some("http://uav.radar-mms.com/elevations".to_string()));
        config.set(ELEVATION_SECTION, "cache_dir",
                   Some(format!("cache{}elevations", MAIN_SEPARATOR).to_string()));
        config.set(ELEVATION_SECTION, "extension", Some("tif".to_string()));
        config.write(DEFAULT_FILENAME).unwrap();
        Self {
          filename: DEFAULT_FILENAME.to_string(),
          ini: config
        }
      }
    }
  }

  pub fn get(&self, section: &str, key: &str) -> Result<String, Error>
  {
    return match self.ini.get(section, key) {
      None => { Err(Error::ConfigMissingKey(key.to_string())) }
      Some(x) => { Ok(x) }
    }
  }
}