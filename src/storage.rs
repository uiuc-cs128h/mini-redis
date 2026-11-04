//input sec(u64)
//storage set needs Duration
//and is stored as Instant
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct Entry {
    value: String,
    expire_at: Option<Instant>,
}

pub struct Storage {
    data: HashMap<String, Entry>,
}

impl Storage {
pub fn new() -> Self {
    Self {
        data: HashMap::new(),
    }
}
pub fn set(&mut self, k: String, v: String, d: Option<Duration>) {
let entry =  Entry {
            value: v,
            expire_at: match d {
                None => None,
                Some(duration) => Some(Instant::now() + duration),
            },
        };
        self.data.insert(k, entry);
    }
//lazy expiraton
pub fn get(&mut self, k: &str) -> Option<String> {
let expired = match self.data.get(k) {
            None => false,
            Some(entry) => match entry.expire_at {
//should instant == instant::now() count as expiration??
                Some(instant) if instant < Instant::now() => true,
                _ => false,
            }
        };
        if expired {
            self.data.remove(k);
        }
        match self.data.get(k) {
            None => None,
            Some(entry) => Some(entry.value.clone()),
        }
    }
pub fn del(&mut self, k: &str) -> bool {
let exist: bool = match self.data.get(k) {
            None => false,
            _ => true,
        };
        if exist {
            self.data.remove(k);
        };
        exist
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_set_and_get() {
        let mut store = Storage::new();
        assert_eq!(store.get("name"), None);

        store.set("name".to_string(), "CS199".to_string(), None);
        assert_eq!(store.get("name"), Some("CS199".to_string()));
    }

    #[test]
    fn test_overwrite_key() {
        let mut store = Storage::new();
        store.set("k".to_string(), "v1".to_string(), None);
        store.set("k".to_string(), "v2".to_string(), None);
        assert_eq!(store.get("k"), Some("v2".to_string()));
    }

    #[test]
    fn test_del() {
        let mut store = Storage::new();
        store.set("k".to_string(), "v1".to_string(), None);
        assert!(store.del("k"));
        assert_eq!(store.get("k"), None);
        assert!(!store.del("k"));
    }

    #[test]
    fn test_expiration() {
        let mut store = Storage::new();
        store.set("temp".to_string(), "data".to_string(), Some(Duration::from_millis(50)),);
        assert_eq!(store.get("temp"), Some("data".to_string()));
        thread::sleep(Duration::from_millis(70));
        assert_eq!(store.get("temp"), None);
    }

    #[test]
    fn test_overwrite_clears_expiration() {
        let mut store = Storage::new();
        store.set("k".to_string(),"v1".to_string(), Some(Duration::from_millis(50)),);
        store.set("k".to_string(), "v2".to_string(), None);

        thread::sleep(Duration::from_millis(70));
        assert_eq!(store.get("k"), Some("v2".to_string()));
    }
}
