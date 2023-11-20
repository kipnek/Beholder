use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Job{
    pub id:String,
    pub command : String,
}

pub struct JobResult{
    pub id:String,
    pub result :String,
}