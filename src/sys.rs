use std::io;

// ==================== UNIX IMPLEMENTATION (macOS / Linux / BSD) ====================

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

#[cfg(unix)]
pub fn bind_socket(fd: RawSocket, ip: &str, port: u16) -> io::Result<()> {
    let ip_addr: std::net::Ipv4Addr = ip
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid IP format"))?;

    let mut sockaddr: libc::sockaddr_in = unsafe { std::mem::zeroed() };
    sockaddr.sin_family = libc::AF_INET as libc::sa_family_t;
    sockaddr.sin_port = port.to_be();
    sockaddr.sin_addr = libc::in_addr {
        s_addr: u32::from(ip_addr).to_be(),
    };

    let res = unsafe {
        libc::bind(
            fd,
            &sockaddr as *const libc::sockaddr_in as *const libc::sockaddr,
            std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
        )
    };

    if res < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(unix)]
pub fn listen_socket(fd: RawSocket, backlog: i32) -> io::Result<()> {
    let res = unsafe { libc::listen(fd, backlog) };
    if res < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(unix)]
pub fn accept_socket(fd: RawSocket) -> io::Result<(RawSocket, String)> {
    let mut client_addr: libc::sockaddr_in = unsafe { std::mem::zeroed() };
    let mut addr_len = std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;

    let client_fd = unsafe {
        libc::accept(
            fd,
            &mut client_addr as *mut libc::sockaddr_in as *mut libc::sockaddr,
            &mut addr_len,
        )
    };

    if client_fd < 0 {
        Err(io::Error::last_os_error())
    } else {
        let ip_u32 = u32::from_be(client_addr.sin_addr.s_addr);
        let ip = std::net::Ipv4Addr::from(ip_u32);
        let port = u16::from_be(client_addr.sin_port);
        Ok((client_fd, format!("{}:{}", ip, port)))
    }
}

#[cfg(unix)]
pub fn connect_socket(fd: RawSocket, ip: &str, port: u16) -> io::Result<()> {
    let ip_addr: std::net::Ipv4Addr = ip
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid IP format"))?;

    let mut sockaddr: libc::sockaddr_in = unsafe { std::mem::zeroed() };
    sockaddr.sin_family = libc::AF_INET as libc::sa_family_t;
    sockaddr.sin_port = port.to_be();
    sockaddr.sin_addr = libc::in_addr {
        s_addr: u32::from(ip_addr).to_be(),
    };

    let res = unsafe {
        libc::connect(
            fd,
            &sockaddr as *const libc::sockaddr_in as *const libc::sockaddr,
            std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
        )
    };

    if res < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(unix)]
pub fn send_bytes(fd: RawSocket, buf: &[u8]) -> io::Result<usize> {
    let res = unsafe { libc::send(fd, buf.as_ptr() as *const libc::c_void, buf.len(), 0) };
    if res < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(res as usize)
    }
}

#[cfg(unix)]
pub fn recv_bytes(fd: RawSocket, buf: &mut [u8]) -> io::Result<usize> {
    let res = unsafe { libc::recv(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len(), 0) };
    if res < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(res as usize)
    }
}

// ==================== WINDOWS IMPLEMENTATION ====================

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
#[repr(C)]
struct IN_ADDR {
    s_addr: u32,
}

#[cfg(windows)]
#[repr(C)]
struct SOCKADDR_IN {
    sin_family: i16,
    sin_port: u16,
    sin_addr: IN_ADDR,
    sin_zero: [i8; 8],
}

#[cfg(windows)]
#[link(name = "ws2_32")]
unsafe extern "system" {
    fn WSAStartup(wVersionRequested: u16, lpWSAData: *mut WSADATA) -> i32;
    fn socket(af: i32, type_: i32, protocol: i32) -> RawSocket;
    fn closesocket(s: RawSocket) -> i32;
    fn bind(s: RawSocket, name: *const SOCKADDR_IN, namelen: i32) -> i32;
    fn listen(s: RawSocket, backlog: i32) -> i32;
    fn accept(s: RawSocket, addr: *mut SOCKADDR_IN, addrlen: *mut i32) -> RawSocket;
    fn connect(s: RawSocket, name: *const SOCKADDR_IN, namelen: i32) -> i32;
    fn send(s: RawSocket, buf: *const i8, len: i32, flags: i32) -> i32;
    fn recv(s: RawSocket, buf: *mut i8, len: i32, flags: i32) -> i32;
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

#[cfg(windows)]
pub fn bind_socket(sock: RawSocket, ip: &str, port: u16) -> io::Result<()> {
    let ip_addr: std::net::Ipv4Addr = ip
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid IP format"))?;

    let sockaddr = SOCKADDR_IN {
        sin_family: AF_INET as i16,
        sin_port: port.to_be(),
        sin_addr: IN_ADDR {
            s_addr: u32::from(ip_addr).to_be(),
        },
        sin_zero: [0; 8],
    };

    let res = unsafe {
        bind(
            sock,
            &sockaddr as *const SOCKADDR_IN,
            std::mem::size_of::<SOCKADDR_IN>() as i32,
        )
    };

    if res != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(windows)]
pub fn listen_socket(sock: RawSocket, backlog: i32) -> io::Result<()> {
    let res = unsafe { listen(sock, backlog) };
    if res != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(windows)]
pub fn accept_socket(sock: RawSocket) -> io::Result<(RawSocket, String)> {
    let mut client_addr = unsafe { std::mem::zeroed::<SOCKADDR_IN>() };
    let mut addr_len = std::mem::size_of::<SOCKADDR_IN>() as i32;

    let client_sock = unsafe { accept(sock, &mut client_addr as *mut SOCKADDR_IN, &mut addr_len) };

    if client_sock == INVALID_SOCKET {
        Err(io::Error::last_os_error())
    } else {
        let ip_u32 = u32::from_be(client_addr.sin_addr.s_addr);
        let ip = std::net::Ipv4Addr::from(ip_u32);
        let port = u16::from_be(client_addr.sin_port);
        Ok((client_sock, format!("{}:{}", ip, port)))
    }
}

#[cfg(windows)]
pub fn connect_socket(sock: RawSocket, ip: &str, port: u16) -> io::Result<()> {
    let ip_addr: std::net::Ipv4Addr = ip
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid IP format"))?;

    let sockaddr = SOCKADDR_IN {
        sin_family: AF_INET as i16,
        sin_port: port.to_be(),
        sin_addr: IN_ADDR {
            s_addr: u32::from(ip_addr).to_be(),
        },
        sin_zero: [0; 8],
    };

    let res = unsafe {
        connect(
            sock,
            &sockaddr as *const SOCKADDR_IN,
            std::mem::size_of::<SOCKADDR_IN>() as i32,
        )
    };

    if res != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(windows)]
pub fn send_bytes(sock: RawSocket, buf: &[u8]) -> io::Result<usize> {
    let res = unsafe { send(sock, buf.as_ptr() as *const i8, buf.len() as i32, 0) };
    if res < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(res as usize)
    }
}

#[cfg(windows)]
pub fn recv_bytes(sock: RawSocket, buf: &mut [u8]) -> io::Result<usize> {
    let res = unsafe { recv(sock, buf.as_mut_ptr() as *mut i8, buf.len() as i32, 0) };
    if res < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(res as usize)
    }
}
