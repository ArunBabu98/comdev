// src/sys.rs
use std::io;

// --- UNIX ---
#[cfg(unix)]
pub type RawSocket = libc::c_int;

#[cfg(unix)]
pub fn create_tcp_socket() -> io::Result<RawSocket> {
    unsafe {
        let fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
        if fd < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(fd)
        }
    }
}

#[cfg(unix)]
pub fn close_socket(fd: RawSocket) {
    unsafe {
        libc::close(fd);
    }
}

// --- WINDOWS ---
#[cfg(windows)]
pub type RawSocket = usize;

#[cfg(windows)]
const AF_INET: i32 = 2;
#[cfg(windows)]
const SOCK_STREAM: i32 = 1;
#[cfg(windows)]
const IPPROTO_TCP: i32 = 6;
#[cfg(windows)]
const INVALID_SOCKET: RawSocket = !0;

#[cfg(windows)]
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

#[cfg(windows)]
#[link(name = "ws2_32")]
extern "system" {
    fn WSAStartup(wVersionRequested: u16, lpWSAData: *mut WSADATA) -> i32;
    fn socket(af: i32, type_: i32, protocol: i32) -> RawSocket;
    fn closesocket(s: RawSocket) -> i32;
}

#[cfg(windows)]
pub fn create_tcp_socket() -> io::Result<RawSocket> {
    unsafe {
        let mut wsa_data = std::mem::zeroed::<WSADATA>();
        let init_res = WSAStartup(0x0202, &mut wsa_data);
        if init_res != 0 {
            return Err(io::Error::from_raw_os_error(init_res));
        }

        let sock = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
        if sock == INVALID_SOCKET {
            Err(io::Error::last_os_error())
        } else {
            Ok(sock)
        }
    }
}

#[cfg(windows)]
pub fn close_socket(sock: RawSocket) {
    unsafe {
        closesocket(sock);
    }
}