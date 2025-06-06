use anyhow::Result;
use sled::Db;

pub trait Storage {
    fn put(&self, key: &str, value: &str) -> Result<()>;
    fn get(&self, key: &str) -> Result<Option<String>>;
    fn list(&self) -> Result<Vec<String>>;
    fn delete(&self, key: &str) -> Result<()>;
}

pub struct SledStorage {
    db: Db,
}

impl SledStorage {
    pub fn new(path: &str) -> Result<SledStorage> {
        let db = sled::open(path)?;
        Ok(SledStorage { db })
    }
}

impl Storage for SledStorage {
    fn put(&self, key: &str, value: &str) -> Result<()> {
        self.db.insert(key.as_bytes(), value.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }

    fn get(&self, key: &str) -> Result<Option<String>> {
        match self.db.get(key.as_bytes())? {
            Some(value) => {
                let value_str =
                    String::from_utf8(value.to_vec()).expect("Non-UTF8 value found in database");
                Ok(Some(value_str))
            }
            None => Ok(None),
        }
    }

    fn list(&self) -> Result<Vec<String>> {
        let mut keys = Vec::new();
        for result in self.db.iter() {
            let (key, _) = result?;
            let key_str = String::from_utf8(key.to_vec()).expect("Non-UTF8 key found in database");
            keys.push(key_str);
        }
        Ok(keys)
    }

    fn delete(&self, key: &str) -> Result<()> {
        self.db.remove(key.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }
}
