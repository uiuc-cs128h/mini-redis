use std::time::Duration;

pub trait Storage {
    fn set(&mut self, key: String, value: String, ttl: Option<Duration>);
    fn get(&mut self, key: &str) -> Option<String>;
    fn delete(&mut self, key: &str) -> bool; //return true if value is removed
}
