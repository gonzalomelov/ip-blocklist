pub mod app_state;
pub mod methods;

use std::collections::HashSet;
use std::error::Error;

use csv::ReaderBuilder;

pub fn read_from_file_to_hash_set(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
  let mut hs = HashSet::new();

  let mut reader = ReaderBuilder::new().has_headers(false).from_path(path)?;

  for result in reader.deserialize() {
      let record: String = result?;
      hs.insert(record);
  }

  Ok(hs)
}