use sled::Db,

lazy_static::lazy_static!{
  static ref DB: Db = sled::open("brdige_db").unwrap()
}

pub fn is_processed(key: &str) -> anyhow::Result<bool> {
  Ok(DB.contains_key(key?))
} 

pub fn mark_processed(key: &str, value: &str) -> anyhow::Result<()> {
    DB.insert(key, value);
    Ok(())
}
