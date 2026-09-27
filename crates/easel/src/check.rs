//! Requests whose client is gone.
//!
//! The server skips a request if its client timed out or died before the request could run,
//! so a `do` the painter gave up on cannot enter the log later behind its back.

use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;

#[repr(C)]
struct PollFd {
    fd: i32,
    events: i16,
    revents: i16,
}
#[cfg(target_os = "linux")]
type NFds = u64;
#[cfg(not(target_os = "linux"))]
type NFds = u32;
unsafe extern "C" {
    fn poll(fds: *mut PollFd, n: NFds, timeout: i32) -> i32;
}
const POLLOUT: i16 = 0x4;
const POLLERR: i16 = 0x8;
const POLLHUP: i16 = 0x10;
const POLLNVAL: i16 = 0x20;

/// Whether the client at the other end of `conn` is gone (it closed, timed out or died), so
/// nobody will read the reply. The client has shut down its sending side after the request,
/// so reading can't tell (end of file either way). macOS reports POLLHUP for both; a socket
/// whose peer has closed is no longer writable. Linux reports POLLHUP only once the peer
/// has closed.
pub fn client_gone(conn: &UnixStream) -> bool {
    let mut p = PollFd { fd: conn.as_raw_fd(), events: POLLOUT, revents: 0 };
    if unsafe { poll(&mut p, 1, 0) } < 0 {
        return false;
    }
    if p.revents & (POLLERR | POLLNVAL) != 0 {
        return true;
    }
    if cfg!(target_os = "linux") { p.revents & POLLHUP != 0 } else { p.revents & POLLHUP != 0 && p.revents & POLLOUT == 0 }
}

#[cfg(test)]
mod tests {
    use super::client_gone;
    use std::io::{Read, Write};
    use std::os::unix::net::UnixStream;

    #[test]
    fn a_client_waiting_for_its_reply_is_there_and_one_that_closed_is_gone() {
        let (mut client, mut server) = UnixStream::pair().unwrap();
        // what a client does: send the request, shut its sending side, wait for the reply
        client.write_all(b"status\n").unwrap();
        client.shutdown(std::net::Shutdown::Write).unwrap();
        let mut req = Vec::new();
        server.read_to_end(&mut req).unwrap();
        assert!(!client_gone(&server), "a waiting client (half closed) was taken for gone");
        drop(client);
        assert!(client_gone(&server), "a closed client was taken for still there");
    }
}
