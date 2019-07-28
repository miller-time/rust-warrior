use std::io;
use std::io::prelude::*;

/// Helper function for prompting the player with a yes/no question
pub fn ask(message: &str) -> bool {
    let mut message = message.to_owned();
    message.push_str(" [y/n] ");
    request(&message).to_lowercase() == "y"
}

// Helper function for prompting the player for a response
pub fn request(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();
    response.trim().to_string()
}
