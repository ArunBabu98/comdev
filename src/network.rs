use crate::sys;
use std::io;
pub struct ComDevnw {
    socket: sys::RawSocket,
}

impl ComDevnw {
    pub fn new() -> io::Result<Self> {
        // Ask OS for TCP socket
        // Bind the socket to an address and port
        // Listen for incoming connections
        // Loop: 1. wait for client 2. accept connection 3. exchange messages 4. close connection when finished

        let socket = sys::create_tcp_socket().expect("Failed to create socket");
        print!("Socket created: {}\n", socket);
        Ok(ComDevnw { socket })
    }
}

// If error happens, using rust RAII, the socket will be closed automatically when the ComDevnw instance goes out of scope. This is done by implementing the Drop trait for ComDevnw.
impl Drop for ComDevnw {
    fn drop(&mut self) {
        sys::close_socket(self.socket);
        println!("Socket closed: {}", self.socket);
    }
}
