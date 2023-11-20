mod config;
mod agent;
mod agent_config;

use crate::agent::Agent;
use crate::agent_config::get_agent_config;
use crate::config::AgentConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent_config = get_agent_config();
    let agent = Agent::new(agent_config);

    loop{


        break;
    }
    Ok(())
}

fn initialize(config:AgentConfig) -> agent::Agent{
    Agent::new(config)
}
