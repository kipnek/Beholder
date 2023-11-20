mod config;
mod agent;
mod agent_config;

use crate::agent::Agent;
use crate::agent_config::get_agent_config;
use crate::config::AgentConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent_config = get_agent_config();
    let mut agent = Agent::new(agent_config);
    if initialize().is_ok() && agent.ping_home().await.is_ok(){
        work(agent).await;
    }
    Ok(())
}

fn initialize() -> Result<(), String>{
    //initialize the agent
    Ok(())
}

async fn work(mut agent:Agent){
    loop{
        //do jobs logic
        break;
    }
}