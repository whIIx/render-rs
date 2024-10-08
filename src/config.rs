pub struct Config {
    pub app_name: String,
    pub width: i32,
    pub height: i32,
}

impl Config {
    pub fn new(app_name: String, width: i32, height: i32) -> Self {
        Self { app_name, width, height }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            app_name: String::from("Rust Renderer"),
            width: 1920,
            height: 1080,
        }
    }
}