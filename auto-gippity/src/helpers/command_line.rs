use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io;

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(
        &self,
        agent_pos: &str,
        agent_statement: &str,
    ) -> anyhow::Result<()> {
        let mut stdout = io::stdout();

        // Decide on the print color
        let statement_color = match self {
            PrintCommand::AICall => Color::Cyan,
            PrintCommand::UnitTest => Color::Magenta,
            PrintCommand::Issue => Color::Red,
        };

        // Print the agent statement
        stdout.execute(SetForegroundColor(Color::Green))?;
        print!("Agent: {}: ", agent_pos);

        // Make select color
        stdout.execute(SetForegroundColor(statement_color))?;
        println!("{}", agent_statement);

        // Reset the color
        stdout.execute(ResetColor)?;

        Ok(())
    }
}

// Get user request
pub fn get_user_response(question: &str) -> anyhow::Result<String> {
    let mut stdout = io::stdout();

    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue))?;
    println!("");
    println!("{}", question);

    // Reset the color
    stdout.execute(ResetColor)?;

    // Read the user input
    let mut user_response = String::new();
    io::stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    Ok(user_response.trim().to_string())
}

pub fn confirm_safe_code() -> anyhow::Result<bool> {
    let mut stdout = io::stdout();

    loop {
        stdout.execute(SetForegroundColor(Color::Blue))?;
        println!("");
        print!("WARNING: You are about to run code written entirely by AI.");
        println!("Review your code and confirm you wish to continue.");

        stdout.execute(SetForegroundColor(Color::Green))?;
        println!("[1] All good");
        stdout.execute(SetForegroundColor(Color::Red))?;
        println!("[2] Lets stop this project");

        stdout.execute(ResetColor)?;

        let mut human_response = String::new();
        io::stdin()
            .read_line(&mut human_response)
            .expect("Failed to read response");

        let human_response = human_response.trim().to_lowercase();

        match human_response.as_str() {
            "1" | "ok" | "y" => return Ok(true),
            "2" | "no" | "n" => return Ok(false),
            _ => {
                println!("Invalid input. Please select '1' or '2'.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prints_agent_msg() {
        let print_command = PrintCommand::AICall;
        let agent_pos = "Managing Agent";
        let agent_statement = "Testing testing, processing something";
        let result = print_command.print_agent_message(agent_pos, agent_statement);
        assert_eq!(result.is_ok(), true);
    }
}
