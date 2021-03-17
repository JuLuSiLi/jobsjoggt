//use crate::components::*;

#[derive(Debug)]
pub struct GameObject {
    name: String,
}

impl GameObject {
    pub fn new(name: &str) -> GameObject {
        GameObject {
            name: name.to_string(),
        }
    }
}