pub fn is_valid_latitude(lat: f64) -> bool
{
  lat >= -90.0 && lat <= 90.0
}

pub fn is_valid_longitude(lon: f64) -> bool
{
  lon >= -180.0 && lon <= 180.0
}

pub fn clip_latitude(lat: f64) -> f64
{
  if lat > 90.0 { return 90.0 }
  else if lat < -90.0 { return -90.0 }
  lat
}

pub fn clip_longitude(lon: f64) -> f64
{
  if lon > 180.0 { return lon - 360.0}
  else if lon < -180.0 { return lon + 360.0 }
  lon
}