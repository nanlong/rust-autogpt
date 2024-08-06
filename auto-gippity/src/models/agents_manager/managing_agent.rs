use crate::models::{
    agent_basic::{basic_agent::BasicAgent, basic_traits::BasicTraits},
    agents::agent_traits::{FactSheet, SpecialFunctions},
};

pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}
