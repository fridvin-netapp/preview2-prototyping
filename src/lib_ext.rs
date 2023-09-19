/*
pub type Fd = u32;
pub type Errno = usize;
pub type Size = usize;
pub type Filesize = u64;
pub type Timestamp = u64;
pub type Clockid = u32;
*/

use wasi::*;

pub type AddressFamily = u8;
pub type SockType = u8;
pub type IpPort = u16;
pub type AddrType = u8;

/*
/// Internet version 4 addresses
pub const ADDRESS_FAMILY_INET4: AddressFamily = 0;
/// Internet version 6 addresses
pub const ADDRESS_FAMILY_INET6: AddressFamily = 1;
/// The file descriptor or file refers to a datagram socket.
pub const SOCK_TYPE_SOCKET_DGRAM: SockType = 0;
/// The file descriptor or file refers to a byte-stream socket.
pub const SOCK_TYPE_SOCKET_STREAM: SockType = 1;
/// IPv4 address
pub const ADDR_TYPE_IP4: AddrType = 0;
/// IPv6 address
pub const ADDR_TYPE_IP6: AddrType = 1;
*/

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AddrIp4 {
    pub n0: u8,
    pub n1: u8,
    pub h0: u8,
    pub h1: u8,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AddrIp4Port {
    pub addr: AddrIp4,
    pub port: IpPort,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AddrIp6 {
    pub n0: u16,
    pub n1: u16,
    pub n2: u16,
    pub n3: u16,
    pub h0: u16,
    pub h1: u16,
    pub h2: u16,
    pub h3: u16,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AddrIp6Port {
    pub addr: AddrIp6,
    pub port: IpPort,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AddrU {
    pub ip4: AddrIp4Port,
    pub ip6: AddrIp6Port,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Addr {
    pub tag: AddrType,
    pub u: AddrU,
}



//extern "C" {
    /// Resolves a hostname and a port to one or more IP addresses. Port is optional
    /// and you can pass 0 (zero) in most cases, it is used a hint for protocol.
    ///
    /// Note: This is similar to `getaddrinfo` in POSIX
    ///
    /// When successful, the contents of the output buffer consist of a sequence of
    /// IPv4 and/or IPv6 addresses. Each address entry consists of a addr_t object.
    /// This function fills the output buffer as much as possible, potentially
    /// truncating the last address entry. It is advisable that the buffer is
    #[no_mangle]
    pub unsafe extern "C" fn addr_resolve(
        host_ptr: *const u8,
        host_len: usize,
        port: IpPort,
        buf: *mut u8,
        buf_len: Size,
        bufused: *mut Size,
    ) -> Errno
    {
        ERRNO_SUCCESS
    }
    /// Returns the local address to which the socket is bound.
    ///
    /// Note: This is similar to `getsockname` in POSIX
    ///
    /// When successful, the contents of the output buffer consist of an IP address,
    /// either IP4 or IP6.
    #[no_mangle]
    pub unsafe extern "C" fn sock_addr_local(fd: Fd, buf: *mut u8, buf_len: Size) -> Errno
    {
        ERRNO_SUCCESS
    }
    /// Returns the remote address to which the socket is connected to.
    ///
    /// Note: This is similar to `getpeername` in POSIX
    ///
    /// When successful, the contents of the output buffer consist of an IP address,
    /// either IP4 or IP6.
    #[no_mangle]
    pub unsafe extern "C" fn sock_addr_remote(fd: Fd, buf: *mut u8, buf_len: Size) -> Errno {
        ERRNO_SUCCESS
    }
    /// Open a socket
    ///
    /// The first argument to this function is a handle to an
    /// address pool. The address pool determines what actions can
    /// be performed and at which addresses they can be performed to.
    ///
    /// The address pool cannot be re-assigned. You will need to close
    /// the socket and open a new one to use a different address pool.
    ///
    /// Note: This is similar to `socket` in POSIX using PF_INET
    #[no_mangle]
    pub unsafe extern "C" fn sock_open(af: AddressFamily, socktype: SockType, fd: *mut Fd) -> Errno{
        ERRNO_SUCCESS
    }
    /// Close a socket (this is an alias for `fd_close`)
    /// Note: This is similar to `close` in POSIX.
    #[no_mangle]
    pub unsafe extern "C" fn sock_close(fd: Fd) -> Errno {
        ERRNO_SUCCESS
    }
    /// Enable/disable address reuse on a socket
    /// Note: This is similar to `setsockopt` in POSIX for SO_REUSEADDR
    #[no_mangle]
    pub unsafe extern "C" fn sock_set_reuse_addr(fd: Fd, reuse: u8) -> Errno{
        ERRNO_SUCCESS
    }
    /// Retrieve status of address reuse on a socket
    /// Note: This is similar to `getsockopt` in POSIX for SO_REUSEADDR
    #[no_mangle]
    pub unsafe extern "C" fn sock_get_reuse_addr(fd: Fd, reuse: *mut u8) -> Errno{
        ERRNO_SUCCESS
    }
    /// Enable port reuse on a socket
    /// Note: This is similar to `setsockopt` in POSIX for SO_REUSEPORT
    #[no_mangle]
    pub unsafe extern "C" fn sock_set_reuse_port(fd: Fd, reuse: u8) -> Errno{
        ERRNO_SUCCESS
    }
    /// Retrieve status of port reuse on a socket
    /// Note: This is similar to `getsockopt` in POSIX for SO_REUSEPORT
    #[no_mangle]
    pub unsafe extern "C" fn sock_get_reuse_port(fd: Fd, reuse: *mut u8) -> Errno{
        ERRNO_SUCCESS
    }
    /// Set size of receive buffer
    /// Note: This is similar to `setsockopt` in POSIX for SO_RCVBUF
    #[no_mangle]
    pub unsafe extern "C" fn sock_set_recv_buf_size(fd: Fd, size: Size) -> Errno{
        ERRNO_SUCCESS
    }
    /// Retrieve the size of the receive buffer
    /// Note: This is similar to `getsockopt` in POSIX for SO_RCVBUF
    #[no_mangle]
    pub unsafe extern "C" fn sock_get_recv_buf_size(fd: Fd, size: *mut Size) -> Errno{
        ERRNO_SUCCESS
    }
    /// Set size of send buffer
    /// Note: This is similar to `setsockopt` in POSIX for SO_SNDBUF
    #[no_mangle]
    pub unsafe extern "C" fn sock_set_send_buf_size(fd: Fd, size: Size) -> Errno{
        ERRNO_SUCCESS
    }
    /// Retrieve the size of the send buffer
    /// Note: This is similar to `getsockopt` in POSIX for SO_SNDBUF
    #[no_mangle]
    pub unsafe extern "C" fn sock_get_send_buf_size(fd: Fd, size: *mut Size) -> Errno{
        ERRNO_SUCCESS
    }
    /// Bind a socket
    /// Note: This is similar to `bind` in POSIX using PF_INET
    #[no_mangle]
    pub unsafe extern "C" fn sock_bind(fd: Fd, addr: *mut Addr) -> Errno{
        ERRNO_SUCCESS
    }
    /// Listen for connections on a socket
    /// Note: This is similar to `listen`
    #[no_mangle]
    pub unsafe extern "C" fn sock_listen(fd: Fd, backlog: Size) -> Errno{
        ERRNO_SUCCESS
    }
    /// Accept a connection on a socket
    /// Note: This is similar to `accept`
    #[no_mangle]
    pub unsafe extern "C" fn sock_accept(fd: Fd, childfd: *mut Fd) -> Errno{
        ERRNO_SUCCESS
    }
    /// Initiate a connection on a socket to the specified address
    /// Note: This is similar to `connect` in POSIX
    #[no_mangle]
    pub unsafe extern "C" fn sock_connect(fd: Fd, addr: *mut Addr) -> Errno{
        ERRNO_SUCCESS
    }
    /// Receive a message from a socket.
    /// Note: This is similar to `recv` in POSIX.
    #[no_mangle]
    pub unsafe extern "C" fn sock_recv(
        fd: Fd,
        buf: *mut u8,
        buf_len: Size,
        flags: Riflags,
        bufused: *mut Size,
    ) -> Errno{
        ERRNO_SUCCESS
    }
    /// Receive a message from a socket.
    ///
    /// The address buffer must be at least the size of addr_t.
    ///
    /// Note: This is similar to `recvfrom` in POSIX.
    #[no_mangle]
    pub unsafe extern "C" fn sock_recv_from(
        fd: Fd,
        buf: *mut u8,
        buf_len: Size,
        addr_buf: *mut u8,
        addr_buf_len: Size,
        flags: Riflags,
        bufused: *mut Size,
    ) -> Errno{
        ERRNO_SUCCESS
    }
    /// Send a message on a socket.
    /// Note: This is similar to `send` in POSIX.
    #[no_mangle]
    pub unsafe extern "C" fn sock_send(
        fd: Fd,
        buf: *mut u8,
        buf_len: Size,
        flags: Siflags,
        bufused: *mut Size,
    ) -> Errno{
        ERRNO_SUCCESS
    }
    /// Send a message on a socket.
    /// Note: This is similar to `sendto` in POSIX.
    #[no_mangle]
    pub unsafe extern "C" fn sock_send_to(
        fd: Fd,
        buf: *mut u8,
        buf_len: Size,
        addr: *mut Addr,
        flags: Siflags,
        bufused: *mut Size,
    ) -> Errno{
        ERRNO_SUCCESS
    }
    /// Shut down socket send and receive channels.
    /// Note: This is similar to `shutdown` in POSIX.
    #[no_mangle]
    pub unsafe extern "C" fn sock_shutdown(fd: Fd, how: Sdflags) -> Errno{
        ERRNO_SUCCESS
    }
//}