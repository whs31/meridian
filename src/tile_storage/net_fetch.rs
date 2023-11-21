use std::collections::HashSet;
use crate::errors::Error;
use crate::tile_storage::tile_signature::TileSignature;

pub struct NetworkFetcher
{
  pub client: reqwest::Client,
  pub base_url: String,
  pub available: HashSet<TileSignature>
}

impl NetworkFetcher
{
  pub fn new(base_url: String) -> Self
  {
    Self
    {
      client: reqwest::Client::new(),
      base_url,
      available: HashSet::new()
    }
  }

  #[tokio::main]
  pub async fn fetch(&mut self) -> Result<(), Error>
  {
    // TODO
    return Ok(());
  }

  //#[tokio::main]
  //pub async fn download_tile(&mut self, signature: &TileSignature) -> Result<(), Error>
}