use rust_extensions::ApplicationStates;

use crate::dictionary::Dictionary;

pub struct AppContext {
    pub dictionary: Dictionary,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            dictionary: Dictionary::new(),
        }
    }
}

impl ApplicationStates for AppContext {
    fn is_initialized(&self) -> bool {
        true
    }

    fn is_shutting_down(&self) -> bool {
        false
    }
}
