type WallClock = std::time::SystemTime;

pub fn wall_clock_now() -> WallClock {
    WallClock::now()
}
