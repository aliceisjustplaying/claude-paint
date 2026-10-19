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
