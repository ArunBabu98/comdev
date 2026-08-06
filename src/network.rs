use crate::sys;
use std::io;

pub struct ComDevnw {
    socket: sys::RawSocket,
}

impl ComDevnw {
    pub fn new() -> io::Result<Self> {
        let socket = sys::create_tcp_socket()?;
        println!("Base socket created: {}", socket);
        Ok(ComDevnw { socket })
    }

    /// Connect to a remote target server
    pub fn connect(&self, ip: &str, port: u16) -> io::Result<()> {
        sys::connect_socket(self.socket, ip, port)?;
        println!("Connected successfully to {}:{}", ip, port);
        Ok(())
    }

    /// Listen on all interfaces (0.0.0.0) and handle incoming connections
    pub fn listen(&self, port: u16) -> io::Result<()> {
        sys::bind_socket(self.socket, "0.0.0.0", port)?;
        sys::listen_socket(self.socket, 5)?;
        println!("Listening on 0.0.0.0:{}...", port);

        // Wait for single incoming client connection
        let (client_socket, client_addr) = sys::accept_socket(self.socket)?;
        println!("Client connected from: {}", client_addr);

        // Chat session loop with the accepted client
        loop {
            match Self::recv_msg(client_socket) {
                Ok(msg) if msg.trim() == "exit" || msg.is_empty() => {
                    println!("Client disconnected.");
                    break;
                }
                Ok(msg) => {
                    println!("Received from client: {}", msg);
                    // Echo message back to client
                    let reply = format!("ACK: {}", msg);
                    let _ = Self::send_msg(client_socket, &reply);
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    break;
                }
            }
        }

        sys::close_socket(client_socket);
        Ok(())
    }

    /// Send string message over a specific raw socket descriptor
    pub fn send_msg(sock: sys::RawSocket, msg: &str) -> io::Result<usize> {
        sys::send_bytes(sock, msg.as_bytes())
    }

    /// Receive string message from a specific raw socket descriptor
    pub fn recv_msg(sock: sys::RawSocket) -> io::Result<String> {
        let mut buffer = [0u8; 1024];
        let bytes_read = sys::recv_bytes(sock, &mut buffer)?;
        if bytes_read == 0 {
            return Ok(String::new()); // Connection closed
        }
        let received_str = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        Ok(received_str)
    }

    pub fn raw_socket(&self) -> sys::RawSocket {
        self.socket
    }
}

impl Drop for ComDevnw {
    fn drop(&mut self) {
        sys::close_socket(self.socket);
        println!("Socket {} dropped and closed.", self.socket);
    }
}
