#![allow(dead_code)]
pub fn sock_of(dir: &std::path::Path) -> std::path::PathBuf {
    use std::os::unix::fs::MetadataExt;
    let p = dir.join("sock");
    if p.as_os_str().len() < 100 {
        return p;
    }
    std::fs::create_dir_all(dir).unwrap();
    let dir = dir.canonicalize().unwrap();
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in dir.as_os_str().as_encoded_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    let uid = std::fs::metadata(&dir).unwrap().uid();
    let file = format!("{h:016x}.sock");
    for base in [std::env::temp_dir(), std::path::PathBuf::from("/tmp")] {
        let private = base.join(format!("easel-{uid}"));
        let s = private.join(&file);
        if s.as_os_str().len() < 100 {
            use std::os::unix::fs::DirBuilderExt;
            let _ = std::fs::DirBuilder::new().mode(0o700).create(&private);
            if let Ok(m) = std::fs::symlink_metadata(&private)
                && m.is_dir() && m.uid() == uid && m.mode() & 0o077 == 0
            {
                return s;
            }
        }
    }
    p
}

/// Stops the server of the session whose directory is `dir` (`<root>/out/easel/<name>`), if one
/// holds the session's lock: asks it to close, and kills it if it still holds the lock 10 s
/// later and the lock names an easel serving this session. For a test's guard: it never panics
/// and needs neither the easel binary nor a reply.
pub fn stop_server(dir: &std::path::Path) {
    use std::{io::{Read, Write}, os::unix::net::UnixStream, time::{Duration, Instant}};
    let held = || std::fs::File::open(dir.join("lock")).is_ok_and(|lock| lock.try_lock().is_err());
    if !held() {
        return;
    }
    if let Ok(mut conn) = UnixStream::connect(sock_of(dir)) {
        let _ = conn.set_read_timeout(Some(Duration::from_secs(10)));
        let _ = conn.set_write_timeout(Some(Duration::from_secs(10)));
        if conn.write_all(b"6\nclose\n").is_ok() {
            let _ = conn.read_to_end(&mut Vec::new());
        }
    }
    let deadline = Instant::now() + Duration::from_secs(10);
    while held() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(20));
    }
    let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or_default();
    let Some(pid) = std::fs::read_to_string(dir.join("lock")).ok().and_then(|p| p.trim().parse::<u32>().ok()).filter(|&p| p > 1) else { return };
    let Ok(ps) = std::process::Command::new("ps").args(["-o", "command=", "-p", &pid.to_string()]).output() else { return };
    let ps = String::from_utf8_lossy(&ps.stdout);
    if held() && matches!(ps.split_whitespace().collect::<Vec<_>>()[..], [easel, "serve", session] if easel.ends_with("easel") && session == name) {
        let _ = std::process::Command::new("kill").args(["-KILL", &pid.to_string()]).status();
    }
}

/// Stops every session's server under the studio `root` (`stop_server`).
pub fn stop_servers(root: &std::path::Path) {
    for dir in std::fs::read_dir(root.join("out/easel")).into_iter().flatten().flatten() {
        stop_server(&dir.path());
    }
}

/// Stops a session's server (its directory, `stop_server`) however the test ends.
pub struct Closing(pub std::path::PathBuf);
impl Drop for Closing {
    fn drop(&mut self) {
        stop_server(&self.0);
    }
}
