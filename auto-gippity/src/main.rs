pub mod ai_functions;
pub mod apis;
pub mod helpers;
pub mod models;

use helpers::command_line::get_user_response;

fn main() -> anyhow::Result<()> {
    let user_req = get_user_response("What webserver are we building today?")?;

    dbg!(user_req);

    Ok(())
}
