use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io;

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
