pub use std::thread::spawn as start_thread;

pub fn spawn_thread() {
    let _handle = start_thread(|| 1_u8);
}
