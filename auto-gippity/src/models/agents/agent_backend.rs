use super::agent_traits::{FactSheet, RouteObject, SpecialFunctions};
use crate::{
    ai_functions::aifunc_backend::{
        print_backend_webserver_code, print_fixed_code, print_improved_webserrver_code,
        print_rest_api_endpoints,
    },
    helpers::{
        command_line::{confirm_safe_code, PrintCommand},
        general::{
            ai_task_request, check_status_code, read_code_template_contents,
            read_exec_main_contents, save_api_endpoints, save_backend_code,
            WEB_SERVER_PROJECT_PATH,
        },
    },
    models::agent_basic::{
        basic_agent::{AgentState, BasicAgent},
        basic_traits::BasicTraits,
    },
};
use reqwest::Client;
use std::{
    process::{Command, Stdio},
    time::Duration,
};
use tokio::time;

#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: usize,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Develops backend code for webserver and json database".to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovery,
            memory: Vec::new(),
        };

        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }

    async fn call_initial_backend_code(&mut self, factsheet: &mut FactSheet) -> anyhow::Result<()> {
        let code_template_str = read_code_template_contents();

        let msg_context = format!(
            "CODE TEMPLATE: {} \n PROJECT DESCRIPTION: {} \n",
            code_template_str, factsheet.project_description
        );

        let ai_response = ai_task_request(
            &msg_context,
            self.attributes.get_position(),
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await?;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);

        Ok(())
    }

    async fn call_improved_backend_code(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> anyhow::Result<()> {
        let msg_context = format!(
            "CODE TEMPLATE: {:?} \n PROJECT DESCRIPTION: {:?} \n",
            factsheet.backend_code, factsheet
        );

        let ai_response = ai_task_request(
            &msg_context,
            self.attributes.get_position(),
            get_function_string!(print_improved_webserrver_code),
            print_improved_webserrver_code,
        )
        .await?;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);

        Ok(())
    }

    async fn call_fix_code_bugs(&mut self, factsheet: &mut FactSheet) -> anyhow::Result<()> {
        let msg_context = format!(
            "BROKEN CODE: {:?} \n ERROR BUGS: {:?} \n
            THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE",
            factsheet.backend_code, self.bug_errors
        );

        let ai_response = ai_task_request(
            &msg_context,
            self.attributes.get_position(),
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await?;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);

        Ok(())
    }

    async fn call_extract_rest_api_endpoints(&self) -> anyhow::Result<String> {
        let backend_code = read_exec_main_contents();
        let msg_context = format!("CODE INPUT: {}", backend_code);

        let ai_response = ai_task_request(
            &msg_context,
            self.attributes.get_position(),
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await?;

        Ok(ai_response)
    }
}

#[async_trait::async_trait]
impl SpecialFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(&mut self, factsheet: &mut FactSheet) -> anyhow::Result<()> {
        while self.attributes.get_state() != &AgentState::Finished {
            match self.attributes.get_state() {
                AgentState::Discovery => {
                    self.call_initial_backend_code(factsheet).await?;
                    self.attributes.update_state(AgentState::Working);
                    continue;
                }
                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_backend_code(factsheet).await?;
                    } else {
                        self.call_fix_code_bugs(factsheet).await?;
                    }

                    self.attributes.update_state(AgentState::UnitTesting);
                    continue;
                }
                AgentState::UnitTesting => {
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.get_position(),
                        "Backend Code Unit Testing: Requesting user input",
                    )?;

                    let is_safe_code = confirm_safe_code()?;

                    if !is_safe_code {
                        panic!("Better go work on some AI alignment instead...");
                    }

                    // Build and Test Code
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.get_position(),
                        "Backend Code Unit Testing: building project...",
                    )?;

                    let build_backend_server = Command::new("cargo")
                        .arg("build")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .expect("Failed to build backend server");

                    if build_backend_server.status.success() {
                        self.bug_count = 0;
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.get_position(),
                            "Backend Code Unit Testing: Test server build successful...",
                        )?;
                    } else {
                        let error = String::from_utf8(build_backend_server.stderr)?;
                        self.bug_count += 1;
                        self.bug_errors = Some(error);

                        if self.bug_count > 2 {
                            PrintCommand::Issue.print_agent_message(
                                self.attributes.get_position(),
                                "Backend Code Unit Testing: Too many bugs found in code",
                            )?;
                            panic!("Error: Too many bugs");
                        }

                        self.attributes.update_state(AgentState::Working);
                        continue;
                    }

                    let api_endpoints_str = self.call_extract_rest_api_endpoints().await?;
                    let api_endpoints =
                        serde_json::from_str::<Vec<RouteObject>>(&api_endpoints_str)
                            .expect("Failed to decode API Endpoints");

                    let checked_endpoints = api_endpoints
                        .iter()
                        .filter(|route| route.method == "GET" && route.is_route_dynamic == "false")
                        .cloned()
                        .collect::<Vec<RouteObject>>();

                    // Store the API Endpoints
                    factsheet.api_endpoint_schema = Some(checked_endpoints.clone());

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.get_position(),
                        "Backend Code Unit Testing: starting web server...",
                    )?;

                    let mut run_backend_server = Command::new("cargo")
                        .arg("run")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("Failed to run backend application");

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.get_position(),
                        "Backend Code Unit Testing: launching tests on server in 5 seconds...",
                    )?;

                    let seconds_sleep = Duration::from_secs(5);
                    time::sleep(seconds_sleep).await;

                    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;

                    // Check status code
                    for endpoint in checked_endpoints {
                        let testing_msg = format!("Testing endpoint '{}'...", endpoint.route);
                        PrintCommand::UnitTest
                            .print_agent_message(self.attributes.get_position(), &testing_msg)?;

                        // Test url
                        let url = format!("http://localhost:8080{}", endpoint.route);
                        match check_status_code(&client, &url).await {
                            Ok(status_code) => {
                                if !(200..=299).contains(&status_code) {
                                    let err_msg = format!(
                                        "WARNING: Failed to call backend url endpoint {} {}",
                                        url, status_code
                                    );

                                    PrintCommand::Issue.print_agent_message(
                                        self.attributes.get_position(),
                                        &err_msg,
                                    )?;
                                }
                            }
                            Err(e) => {
                                // kill $(lsof -t -i:8080)
                                run_backend_server
                                    .kill()
                                    .expect("Failed to kill backend web server");
                                let err_msg = format!("Error checking backend {}", e);
                                PrintCommand::Issue.print_agent_message(
                                    self.attributes.get_position(),
                                    &err_msg,
                                )?;
                            }
                        }
                    }

                    save_api_endpoints(&api_endpoints_str);

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.get_position(),
                        "Backend testing complete...",
                    )?;

                    run_backend_server
                        .kill()
                        .expect("Failed to kill backend web server on completion");

                    self.attributes.update_state(AgentState::Finished);
                }
                AgentState::Finished => {}
            }
        }

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::models::agents::agent_traits::ProjectScope;

//     use super::*;

//     #[tokio::test]
//     async fn test_writing_backend_code() -> anyhow::Result<()> {
//         let mut agent = AgentBackendDeveloper::new();

//         let mut factsheet = FactSheet {
//             project_description: "build a website that handles users logging in and logging out and displays the latest Forex prices".to_string(),
//             project_scope: Some(ProjectScope {
//                 is_curd_required: false,
//                 is_user_login_and_logout: true,
//                 is_external_urls_required: true,
//             },),
//             external_urls: Some(vec!["https://api.exchangeratesapi.io/latest?base=USD".to_string(),],),
//             backend_code: None,
//             api_endpoint_schema: None,
//         };

//         agent.execute(&mut factsheet).await?;

//         dbg!(&factsheet);

//         Ok(())
//     }
// }
