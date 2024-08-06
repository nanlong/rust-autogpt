use super::command_line::PrintCommand;
use crate::{apis::call_request::call_gpt, models::general::llm::Message};
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fs;

const CODE_TEMPLATE_PATH: &str = "template/code_template.rs";
const EXEC_MAIN_PATH: &str = "template/main.rs";
const API_SCHEMA_PATH: &str = "schema/api_schema.json";
pub const WEB_SERVER_PROJECT_PATH: &str = "template";

// Extend ai function to encourage specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    // Extend the string to encourage only printing the output
    let msg = format!(
        "Function {}
    Instruction: You are a function printer. You ONLY print the results of functions.
    Nothing else. No commentary. Here is the input to the function: {}.
    Print out what the function will return.",
        ai_function_str, func_input
    );

    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// Performs call to LLM GPT
pub async fn ai_task_request(
    msg_context: &str,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> anyhow::Result<String> {
    // Extend ai function
    let extended_msg = extend_ai_function(function_pass, msg_context);
    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation)?;

    // Get LLM response
    let llm_response_res = call_gpt(vec![extended_msg.clone()]).await;

    let llm_response = match llm_response_res {
        Ok(response) => response,
        Err(_) => call_gpt(vec![extended_msg])
            .await
            .expect("Failed twice to call OpenAI"),
    };

    Ok(llm_response)
}

// Performs call to LLM GPT - Decode
pub async fn ai_task_request_decode<T: DeserializeOwned>(
    msg_context: &str,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> anyhow::Result<T> {
    let llm_response =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await?;
    let decoded_response = serde_json::from_str(&llm_response)?;
    Ok(decoded_response)
}

// Check whether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> anyhow::Result<u16> {
    let response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get Code Template
pub fn read_code_template_contents() -> String {
    let path = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

// Get Exec Main
pub fn read_exec_main_contents() -> String {
    let path = String::from(EXEC_MAIN_PATH);
    fs::read_to_string(path).expect("Failed to read main file")
}

// Save New Backend Code
pub fn save_backend_code(backend_code: &str) {
    let path = String::from(EXEC_MAIN_PATH);
    fs::write(path, backend_code).expect("Failed to write main.rs file");
}

// Save JSON API Endpoint Schema
pub fn save_api_endpoints(api_endpoints: &str) {
    let path = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write api_schema.json file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extending_ai_function() {
        use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

        let extended_str = extend_ai_function(convert_user_input_to_goal, "dummy variable");
        dbg!(&extended_str);
        assert_eq!(extended_str.role, "system");
    }

    // #[tokio::test]
    // async fn test_ai_task_request() -> anyhow::Result<()> {
    //     let ai_func_param = "Build me a websserver for making stock price api requests.";

    //     let res = ai_task_request(
    //         ai_func_param,
    //         "Managing Agent",
    //         "Defining user requirements",
    //         convert_user_input_to_goal,
    //     )
    //     .await?;

    //     dbg!(&res);

    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_ai_task_request_decode() -> anyhow::Result<()> {
    //     use crate::ai_functions::aifunc_architect::print_project_scope;
    //     use crate::models::agents::agent_traits::ProjectScope;

    //     let msg_context = "Build me a websserver for making stock price api requests.";

    //     let ai_response = ai_task_request_decode::<ProjectScope>(
    //         &msg_context,
    //         "Solution Architect",
    //         get_function_string!(print_project_scope),
    //         print_project_scope,
    //     )
    //     .await?;

    //     dbg!(&ai_response);

    //     Ok(())
    // }
}
