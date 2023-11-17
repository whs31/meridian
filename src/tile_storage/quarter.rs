use crate::errors::Error;

#[derive(Debug, PartialEq)]
pub enum Quarter
{
  TopLeft = 0,
  TopRight = 1,
  BottomLeft = 2,
  BottomRight = 3
}

impl Quarter
{
  pub fn from_str(value: &str) -> Result<Self, Error>
  {
    if value.len() != 1 { return Err(Error::InvalidQuarterDirectorySpecifier) }
    let as_int = match value.parse::<u8>() {
      Ok(x) => x,
      Err(_) => return Err(Error::InvalidQuarterDirectorySpecifier)
    };
    return match as_int {
      0 => Ok(Quarter::TopLeft),
      1 => Ok(Quarter::TopRight),
      2 => Ok(Quarter::BottomLeft),
      3 => Ok(Quarter::BottomRight),
      _ => Err(Error::InvalidQuarterDirectorySpecifier)
    }
  }

  pub fn signs(&self) -> (i8, i16)
  {
    return match self {
      Quarter::TopLeft => (1, -1),
      Quarter::TopRight => (1, 1),
      Quarter::BottomLeft => (-1, -1),
      Quarter::BottomRight => (-1, 1)
    }
  }

  pub fn to_u8(&self) -> u8
  {
    return match self {
      Quarter::TopLeft => 0,
      Quarter::TopRight => 1,
      Quarter::BottomLeft => 2,
      Quarter::BottomRight => 3
    }
  }
}