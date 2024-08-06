// agent architect

use crate::{
    ai_functions::aifunc_architect::{print_project_scope, print_site_urls},
    helpers::{
        command_line::PrintCommand,
        general::{ai_task_request_decode, check_status_code},
    },
    models::agent_basic::{
        basic_agent::{AgentState, BasicAgent},
        basic_traits::BasicTraits,
    },
};
use reqwest::Client;
use std::time::Duration;

use super::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

// Solutions Architect
#[derive(Debug)]
pub struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Gathers information and design solutions for website development"
                .to_string(),
            position: "Solution Architect".to_string(),
            state: AgentState::Discovery,
            memory: Vec::new(),
        };

        Self { attributes }
    }

    // Retrieve Project Scope
    async fn call_project_scope(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> anyhow::Result<ProjectScope> {
        let msg_context = format!("{}", factsheet.project_description);

        // agent_position 和 agent_operation 只是用来打印日志的
        let ai_response = ai_task_request_decode::<ProjectScope>(
            &msg_context,
            &self.attributes.position,
            get_function_string!(print_project_scope),
            print_project_scope,
        )
        .await?;

        factsheet.project_scope = Some(ai_response.clone());

        Ok(ai_response)
    }

    // Retrieve External Urls
    async fn call_determine_external_urls(
        &mut self,
        factsheet: &mut FactSheet,
        msg_context: &str,
    ) -> anyhow::Result<()> {
        // agent_position 和 agent_operation 只是用来打印日志的
        let ai_response = ai_task_request_decode::<Vec<String>>(
            &msg_context,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls,
        )
        .await?;

        factsheet.external_urls = Some(ai_response);

        Ok(())
    }
}

#[async_trait::async_trait]
impl SpecialFunctions for AgentSolutionArchitect {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(&mut self, factsheet: &mut FactSheet) -> anyhow::Result<()> {
        while self.attributes.get_state() != &AgentState::Finished {
            match self.attributes.get_state() {
                AgentState::Discovery => {
                    let project_scope = self.call_project_scope(factsheet).await?;

                    if project_scope.is_external_urls_required {
                        self.call_determine_external_urls(
                            factsheet,
                            &factsheet.project_description.clone(),
                        )
                        .await?;

                        self.attributes.update_state(AgentState::UnitTesting);
                    } else {
                        self.attributes.update_state(AgentState::Finished);
                    }
                }

                AgentState::UnitTesting => {
                    let mut exclude_urls = Vec::new();

                    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;

                    let urls = factsheet
                        .external_urls
                        .as_ref()
                        .expect("No URL object in factsheet");

                    for url in urls {
                        let endpoint = format!("Testing URL Endpoint: {}", url);
                        PrintCommand::UnitTest
                            .print_agent_message(self.attributes.get_position(), &endpoint)?;

                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    exclude_urls.push(url.clone());
                                }
                            }
                            Err(e) => println!("Error Checking {}: {}", url, e),
                        }
                    }

                    if exclude_urls.len() > 0 {
                        let new_urls = urls
                            .iter()
                            .filter(|&url| !exclude_urls.contains(url))
                            .cloned()
                            .collect();

                        factsheet.external_urls = Some(new_urls);
                    }

                    self.attributes.update_state(AgentState::Finished);
                }

                _ => {
                    self.attributes.update_state(AgentState::Finished);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[tokio::test]
    // async fn test_solution_architect() -> anyhow::Result<()> {
    //     let mut agent = AgentSolutionArchitect::new();
    //     let project_description =
    //         "Build a full stack website with user login and logout that shows latest Forex prices"
    //             .to_string();
    //     let mut factsheet = FactSheet {
    //         project_description,
    //         ..Default::default()
    //     };

    //     agent.execute(&mut factsheet).await?;
    //     assert!(factsheet.project_scope.is_some());
    //     assert!(factsheet.external_urls.is_some());

    //     dbg!(&factsheet);

    //     Ok(())
    // }
}
