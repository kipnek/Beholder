use std::collections::VecDeque;
use std::pin::pin;
use reqwest::{Error, Response};
use common::job::Job;
use crate::config::AgentConfig;

pub struct Agent{
    pub config:AgentConfig,
    pub jobs : VecDeque<Job>,
}
impl Agent{
    pub fn new(config:AgentConfig)->Self{
        Self{
            config,
            jobs:VecDeque::new()
        }
    }
    pub async fn get_jobs(&mut self) ->Result<(), reqwest::Error>{
        let jobs_result:Result<Vec<Job>, _> = reqwest::get(format!("{}/{}/jobs", self.config.home, self.config.id)).await?.json().await;
        match jobs_result{
            Ok(jobs) => {
                for job in jobs{
                    self.jobs.push_back(job);
                }
                Ok(())
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn ping_home(&mut self) ->Result<(), reqwest::Error>{
        let ping_result = reqwest::get(format!("{}/{}/ping", self.config.home, self.config.id)).await;
        println!("{:?}", ping_result);
        match ping_result{
            Ok(_) => {
                Ok(())
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}
