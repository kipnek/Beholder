use crate::config::AgentConfig;

pub fn get_agent_config() ->AgentConfig{
    AgentConfig{
        id: "{{ID}}".to_string(),
        home: "{{HOME}}".to_string(),
    }
}