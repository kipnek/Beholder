mod config;
mod agent;

use std::collections::HashMap;
use crate::config::AgentConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*let resp = reqwest::get("http://127.0.0.1:8080/ping/1")
        .await?;
    println!("{:?}", resp.text().await?);*/
    loop{

        break;
    }
    Ok(())
}

fn initialize(config:AgentConfig)