use crate::dictionary::DictionaryError;

pub enum FlowsError {
    KeyExists,
    KeyNotFound,
}

impl From<DictionaryError> for FlowsError {
    fn from(src: DictionaryError) -> Self {
        match src {
            DictionaryError::KeyAlreadyExists => FlowsError::KeyExists,
            DictionaryError::KeyNotFound => FlowsError::KeyNotFound,
        }
    }
}
