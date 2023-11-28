use crate::config::AgentConfig;

pub fn get_agent_config() ->AgentConfig{
    AgentConfig{
        id: "8080".to_string(),
        home: "http://127.0.0.1:8080".to_string(),
    }
}