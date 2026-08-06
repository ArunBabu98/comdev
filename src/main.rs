mod node;
use node::Node;
use std::io;

fn main() {
    println!("ComDev Started..........");
    println!("Choose: 1. Listen 2. Connect");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    match choice.trim() {
        "1" => {
            println!("Listening...");
            // Add listening logic here
        },
        "2" => {
            println!("Connecting...");
            // Add connecting logic here
        },
        _ => {
            println!("Invalid choice. Please enter 1 or 2.");
        }
        
    }
}
