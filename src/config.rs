pub struct Config {
    pub view: bool
}

impl Config {
    pub fn get() -> Self {
        Config { view: true }
    }
}
