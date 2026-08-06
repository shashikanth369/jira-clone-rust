use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub enum Status{
    Open,
    InProgress,
    Resolved,
    Closed, 
    Cancelled
}