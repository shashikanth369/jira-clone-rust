pub mod database {
use std::fs;
use anyhow::Result;
use crate::model::{db_state::{self, DBState}, epic::Epic, status::Status, story::Story};

pub struct JiraDatabase {
    database: Box<dyn Database>,
}


impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        JiraDatabase { database: Box::new(JsonFileDatabase::new(file_path)) }
    }

    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_database()
    }
    
    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
       let mut db_state =  self.read_db()?;
       Ok(db_state.insert_epic(epic))
    }
    
    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut db_state = self.read_db()?;
        db_state.insert_story(story, epic_id)
    }
    
    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
          let mut db_state =  self.read_db()?;  
          db_state.delete_epid_by_id(epic_id);
          Ok(())
    }
    
    pub fn delete_story(&self,epic_id: u32, story_id: u32) -> Result<()> {
        let mut db_state =  self.read_db()?;  
        db_state.delete_story_by_id(story_id, epic_id);
        Ok(())
    }
    
    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        self.read_db()?.update_epic_status(epic_id, status);
        Ok(())
    }
    
    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        self.read_db()?.update_story_status(story_id, status);
        Ok(())
    }
}

pub trait Database{
    fn read_database(&self) ->Result<DBState>;
    fn write_database(&self, db_state: &DBState) -> Result<()>;
}

pub struct JsonFileDatabase {
    pub file_path: String,
}

impl JsonFileDatabase {
    pub fn new(file_path: String) -> Self {
        JsonFileDatabase { file_path }
    }
}

impl Database for JsonFileDatabase {

    fn read_database(&self) ->Result<DBState> {
        let db_json = fs::read_to_string(self.file_path.to_owned())?;
        let db_state: DBState = serde_json::from_str(&db_json)?;
            
        Ok(db_state)
    }

    fn write_database(&self, db_state: &DBState) -> Result<()> {
        let json_str = serde_json::to_string(db_state)?;

        fs::write(self.file_path.to_owned(), json_str)?;
        
        Ok(())
    }
}
    
}