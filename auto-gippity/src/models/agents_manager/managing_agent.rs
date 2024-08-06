use crate::{
    ai_functions::aifunc_managing::convert_user_input_to_goal,
    helpers::general::ai_task_request,
    models::{
        agent_basic::basic_agent::{AgentState, BasicAgent},
        agents::{
            agent_architect::AgentSolutionArchitect,
            agent_traits::{FactSheet, SpecialFunctions},
        },
    },
};

#[allow(unused)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}

impl ManagingAgent {
    pub async fn try_new(user_req: &str) -> anyhow::Result<Self> {
        let position = "Project Manager".to_string();

        let attributes = BasicAgent {
            objective: "Manage agents who are building an excellent for the user".to_string(),
            position: position.clone(),
            state: AgentState::Discovery,
            memory: Vec::new(),
        };

        let project_description = ai_task_request(
            user_req,
            &position,
            get_function_string!(convert_user_input_to_goal),
            convert_user_input_to_goal,
        )
        .await?;

        let factsheet = FactSheet {
            project_description,
            ..Default::default()
        };

        Ok(Self {
            attributes,
            factsheet,
            agents: Vec::new(),
        })
    }

    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }

    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));
        // todo: add backend agent
    }

    pub async fn execute_project(&mut self) -> anyhow::Result<()> {
        self.create_agents();

        for agent in self.agents.iter_mut() {
            agent.execute(&mut self.factsheet).await?;
        }

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_managing_agent() -> anyhow::Result<()> {
//         // let user_req =
//         //     "need a full stack app that fetches and tracks my fitness progress. Needs to include timezone into from the web.";
//         let user_req =
//             "Build a full stack website with user login and logout that shows latest Forex prices";
//         let mut managing_agent = ManagingAgent::try_new(user_req).await?;

//         managing_agent.execute_project().await?;
//         dbg!(&managing_agent.factsheet);

//         Ok(())
//     }
// }
