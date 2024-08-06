#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {
        stringify!($func)
    };
}

#[macro_use]
pub mod ai_functions;
pub mod apis;
pub mod helpers;
pub mod models;

use helpers::command_line::get_user_response;
use models::agents_manager::managing_agent::ManagingAgent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let user_req = get_user_response("What website are we building today?")?;
    let mut managing_agent = ManagingAgent::try_new(&user_req).await?;

    managing_agent.execute_project().await?;

    Ok(())
}
