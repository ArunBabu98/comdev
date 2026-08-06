mod network;
mod node;
mod sys;
use std::io::{self, Write};

fn main() {
    println!("ComDev Started..........");

    loop {
        println!("\nChoose: 1. Listen 2. Connect 3. Exit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Starting Server...");
                let network = network::ComDevnw::new().expect("Failed to create network instance");

                // Starts blocking listen & echo loop on port 8080
                if let Err(e) = network.listen(8080) {
                    eprintln!("Server error: {}", e);
                }
            }
            "2" => {
                println!("Starting Client...");
                let network = network::ComDevnw::new().expect("Failed to create network instance");

                print!("Enter Server IP (e.g., 127.0.0.1 or 192.168.1.X): ");
                io::stdout().flush().unwrap();
                let mut ip = String::new();
                io::stdin().read_line(&mut ip).unwrap();

                if let Err(e) = network.connect(ip.trim(), 8080) {
                    eprintln!("Failed to connect: {}", e);
                    continue;
                }

                // Interactive message sending loop
                loop {
                    print!("Message to send (type 'exit' to quit): ");
                    io::stdout().flush().unwrap();

                    let mut msg = String::new();
                    io::stdin().read_line(&mut msg).unwrap();
                    let trimmed = msg.trim();

                    if let Err(e) = network::ComDevnw::send_msg(network.raw_socket(), trimmed) {
                        eprintln!("Send error: {}", e);
                        break;
                    }

                    if trimmed == "exit" {
                        break;
                    }

                    // Receive reply back from server
                    match network::ComDevnw::recv_msg(network.raw_socket()) {
                        Ok(reply) => println!("Server reply: {}", reply),
                        Err(e) => {
                            eprintln!("Receive error: {}", e);
                            break;
                        }
                    }
                }
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
