pub mod model;
pub mod database;
use std::{collections::HashMap, time::Instant};

use crate::{database::database::{Database, JsonFileDatabase}, model::{db_state::DBState, epic::Epic, story::Story}};
fn main() {

    let start = Instant::now();
    let json_database = JsonFileDatabase::new("src/data/db.json".to_owned());

    let db_state = json_database.read_database();

    match db_state{
        Ok(db) => println!("Current DB state: {:#?}", db),
        Err(e) => println!("Error while reading DB: {:?}", e),
    }

    let story1 = Story::new("story-rust-ownership".to_owned(), 
                                "Learn rust ownership and borrow".to_owned());
    let epic1 = Epic::new("Epic-2026-1".to_owned(), "Learn rust zero to hero".to_owned());

    let stories = HashMap::from([(1_u32, story1)]);
    let epics = HashMap::from([(2_u32, epic1)]);

    let db_state = DBState::new(4_u32, epics, stories);
    json_database.write_database(&db_state);

    println!("Time taken: {:?}", start.elapsed());
}       
