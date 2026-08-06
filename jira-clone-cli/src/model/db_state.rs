use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;
use crate::model::{epic::{self, Epic}, status::{self, Status}, story::Story};

#[derive(Debug, Deserialize, Serialize)]
pub struct DBState {
    last_item_id: u32,
    epics: HashMap<u32, Epic>,
    stories: HashMap<u32, Story>
}

impl DBState {
    pub fn new(last_item_id: u32, epics: HashMap<u32, Epic>, stories: HashMap<u32, Story>) -> Self {
        DBState{
            last_item_id: last_item_id,
            epics: epics,
            stories: stories,
        }
    }

    pub fn get_last_item_id(&self) -> u32 {
        self.last_item_id
    }

    pub fn insert_epic(&mut self, epic: Epic) -> u32 {
        let id = self.epics.keys().max().unwrap();
        let new_id = *id+1;
        self.epics.insert(new_id, epic);
        new_id
    }

    pub fn insert_story(&mut self, story: Story, epic_id: u32) -> Result<u32> {
        let id = self.stories.keys().max().unwrap();
        let new_id = *id + 1;
        self.stories.insert(new_id, story);
        let epic = self.epics.get_mut(&epic_id)
            .ok_or_else(|| anyhow::anyhow!("epic not found for epic id {}", epic_id))?;
        epic.update_stories_for_epic(new_id);
        Ok(new_id)
    }

    pub fn delete_epid_by_id(&mut self, epic_id: u32) {
        self.epics.remove_entry(&epic_id);
    }

    pub fn delete_story_by_id(&mut self, story_id: u32, epic_id: u32) {
        self.stories.remove_entry(&story_id);
        self.epics.get_mut(&epic_id).unwrap().delete_story_id_for_epic(story_id);  
    }

    pub fn update_epic_status(&mut self, epic_id: u32, status:Status) {
        self.epics.get_mut(&epic_id).unwrap().update_status(status);
    }

    pub fn update_story_status(&mut self, story_id: u32, status:Status) {
        self.stories.get_mut(&story_id).unwrap().update_status(status);
    }
}