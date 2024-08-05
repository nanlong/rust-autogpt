use crate::models::general::llm::Message;

use super::basic_traits::BasicTraits;

#[derive(Debug, PartialEq)]
pub enum AgentState {
    // 初始状态都是发现
    Discovery,
    Working,
    UnitTesting,
    Finished,
}

pub struct BasicAgent {
    // 任务目标
    pub objective: String,
    // 职位
    pub position: String,
    // 当前状态
    pub state: AgentState,
    // 历史对话
    pub memory: Vec<Message>,
}

impl BasicTraits for BasicAgent {
    fn new(objective: String, position: String) -> Self {
        BasicAgent {
            objective,
            position,
            state: AgentState::Discovery,
            memory: Vec::new(),
        }
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }

    fn get_objective(&self) -> &String {
        &self.objective
    }

    fn get_position(&self) -> &String {
        &self.position
    }

    fn get_state(&self) -> &AgentState {
        &self.state
    }

    fn get_memory(&self) -> &Vec<Message> {
        &self.memory
    }
}
