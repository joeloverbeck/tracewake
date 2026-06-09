macro_rules! spawn_and_read {
    ($path:expr) => {{
        let _handle = std::thread::spawn(|| 1_u8);
        let _bytes = std::fs::read($path);
    }};
}

pub fn macro_expansion(path: &std::path::Path) {
    spawn_and_read!(path);
}
