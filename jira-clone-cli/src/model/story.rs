use serde::{Deserialize, Serialize};
use crate::model::status::Status;

#[derive(Debug, Deserialize, Serialize)]
pub struct Story{
    name: String,
    description: String,
    status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self{
        Story{name: name, description: description, status: Status::Open}
    }

    pub fn update_status(&mut self, status: Status) {
        self.status = status;
    }
}
