use std::io;
// Ask OS for TCP socket
// Bind the socket to an address and port
// Listen for incoming connections
// Loop: 1. wait for client 2. accept connection 3. exchange messages 4. close connection when finished
pub fn listen_nw() {
    let socket = sys::create_tcp_socket().expect("Failed to create socket");
}

// Unix Submodule (macOS / Linux / BSD)
#[cfg(unix)]
mod sys {
    use super::*;

    /// On Unix, socket descriptors are signed 32-bit integers.
    pub type RawSocket = libc::c_int;

    pub fn create_tcp_socket() -> io::Result<RawSocket> {
        unsafe {
            // IPv4 stream socket, 0 allows OS to choose protocol (TCP)
            let fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
            if fd < 0 {
                return Err(io::Error::last_os_error());
            } else {
                Ok(fd)
            }
        }
    }

    pub fn close_socket(fd: RawSocket) {
        unsafe {
            libc::close(fd);
        }
    }
}

// Windows Submodule
#[cfg(windows)]
mod sys {
    use super::*;

    /// On Windows, socket handles are pointer-sized unsigned integers (SOCKET).
    pub type RawSocket = usize;

    const AF_INET: i32 = 2;
    const SOCK_STREAM: i32 = 1;
    const IPPROTO_TCP: i32 = 6;
    const INVALID_SOCKET: RawSocket = !0; // Equivalent to usize::MAX

    #[repr(C)]
    struct WSADATA {
        w_version: u16,
        w_high_version: u16,
        sz_description: [u8; 257],
        sz_system_status: [u8; 129],
        i_max_sockets: u16,
        i_max_udp_ndg: u16,
        lp_vendor_info: *mut u8,
    }

    // Direct foreign function interface to Windows Sockets (ws2_32.dll)
    #[link(name = "ws2_32")]
    extern "system" {
        fn WSAStartup(wVersionRequested: u16, lpWSAData: *mut WSADATA) -> i32;
        fn socket(af: i32, type_: i32, protocol: i32) -> RawSocket;
        fn closesocket(s: RawSocket) -> i32;
    }

    pub fn create_tcp_socket() -> io::Result<RawSocket> {
        unsafe {
            // Step 1: Winsock initialization (version 2.2 = 0x0202)
            let mut wsa_data = std::mem::zeroed::<WSADATA>();
            let init_res = WSAStartup(0x0202, &mut wsa_data);
            if init_res != 0 {
                return Err(io::Error::from_raw_os_error(init_res));
            }

            // Step 2: Request socket from Windows
            let sock = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
            if sock == INVALID_SOCKET {
                Err(io::Error::last_os_error())
            } else {
                Ok(sock)
            }
        }
    }

    pub fn close_socket(sock: RawSocket) {
        unsafe {
            closesocket(sock);
        }
    }
}
