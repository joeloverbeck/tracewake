pub fn ambient_randomness() -> u8 {
    let _rng = rand::rng();
    rand::random::<u8>()
}
