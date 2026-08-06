mod network;
mod node;
mod sys;
use node::Node;
use std::io;

fn main() {
    println!("ComDev Started..........");

    // Declaring outside, so it stays alive
    let mut network_instance = None;

    loop {
        println!("Choose: 1. Listen 2. Connect 3. Exit");

        // Re-initialize choice each iteration so previous inputs aren't appended
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Listening...");
                // Store in outer variable so it isn't immediately dropped
                network_instance = Some(
                    network::ComDevnw::new().expect("Failed to create network instance")
                );
            }
            "2" => {
                println!("Connecting...");
                // Add connecting logic here
            }
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2, or 3.");
            }
        }
    }
}