// src/logger.rs
pub trait Logger {
    fn log(&mut self, message: &str);
}