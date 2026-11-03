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