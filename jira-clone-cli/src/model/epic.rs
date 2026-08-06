use serde::{Deserialize, Serialize};
use crate::model::status::{self, Status};
#[derive(Debug, Deserialize, Serialize)]
pub struct Epic {
    name: String,
    description: String,
    status: Status,
    stories: Vec<u32>
}

impl Epic {
    pub fn new(name: String, description: String) -> Self{
        Epic{name: name, description: description, status: Status::Open, stories: vec![]}
    }

    pub fn update_stories_for_epic(&mut self, story_id: u32) {
        self.stories.push(story_id);
    }

    pub fn delete_story_id_for_epic(&mut self, story_id: u32) {
        if let Some(pos) = self.stories.iter().position(|x| *x == story_id) {
            self.stories.swap_remove(pos);
        }
    }

    pub fn update_status(&mut self, status:Status) {
        self.status = status;
    }
}