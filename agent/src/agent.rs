use common::job::Job;
use crate::config::AgentConfig;

pub struct Agent{
    pub config:AgentConfig,
    pub jobs : Vec<Job>,
}
impl Agent{
    pub fn new(config:AgentConfig)->Self{
        Self{
            config,
            jobs:vec![]
        }
    }
}
