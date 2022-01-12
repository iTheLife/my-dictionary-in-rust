use crate::dictionary::Dictionary;

#[derive(Clone)]
pub struct State {
    //you can add other services
    pub dictionary: Dictionary,
}

impl State {
    pub fn init() -> State {
        State {
            dictionary: Dictionary::new(),
        }
    }
}
