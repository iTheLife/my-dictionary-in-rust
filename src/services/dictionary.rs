use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Dictionary {
    dictionary: Arc<Mutex<HashMap<String, String>>>,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary {
            dictionary: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.dictionary.lock().unwrap().insert(key, value);
    }

    pub fn remove(&mut self, key: &String) {
        self.dictionary.lock().unwrap().remove(key);
    }

    pub fn get(&self, key: &str) -> String {
        match self.dictionary.lock().unwrap().get(key) {
            Some(value) => value.to_string(),
            None => return format!("Значение по ключу: {} не найдено", key),
        }
    }

    pub fn get_all(&self) -> String {
        let mut result = String::new();

        for (key, value) in self.dictionary.lock().unwrap().iter_mut() {
            result.push_str(format!("key: {}, value: {} \n", key, value).as_str());
        }

        result
    }
}
