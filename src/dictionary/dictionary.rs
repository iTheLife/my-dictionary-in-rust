use std::collections::HashMap;

use tokio::sync::RwLock;

use super::DictionaryError;

pub struct DictionaryData {
    data: HashMap<String, String>,
}

impl DictionaryData {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) -> Result<(), DictionaryError> {
        if self.data.contains_key(&key) {
            return Err(DictionaryError::KeyAlreadyExists);
        }

        self.data.insert(key, value);

        Ok(())
    }

    pub fn get(&self, key: String) -> Result<String, DictionaryError> {
        if !self.data.contains_key(&key) {
            return Err(DictionaryError::KeyNotFound);
        }

        let value = self.data.get(&key);

        match value {
            Some(value) => Ok(String::from(value)),
            None => Err(DictionaryError::KeyNotFound),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<(), DictionaryError> {
        if !self.data.contains_key(&key) {
            return Err(DictionaryError::KeyNotFound);
        }

        self.data.remove(&key);

        Ok(())
    }
}

pub struct Dictionary {
    dictionary: RwLock<DictionaryData>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            dictionary: RwLock::new(DictionaryData::new()),
        }
    }

    pub async fn insert(&self, key: String, value: String) -> Result<(), DictionaryError> {
        let mut write_access = self.dictionary.write().await;

        write_access.insert(key, value)
    }

    pub async fn get(&self, key: String) -> Result<String, DictionaryError> {
        let read_access = self.dictionary.read().await;

        read_access.get(key)
    }

    pub async fn remove(&self, key: String) -> Result<(), DictionaryError> {
        let mut write_access = self.dictionary.write().await;

        write_access.remove(key)
    }
}
