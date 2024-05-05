use std::collections::HashMap;
use super::structs::{RecordType, QueryResult, Errors};
use std::time::{Instant, Duration};

pub struct CacheEntry {
    results: Result<Vec<QueryResult>, Errors>,
    timestamp: Instant,
}

pub struct Cache {
    cache: HashMap<RecordType, CacheEntry>,
    age: Duration,
}

impl Cache {
    pub fn new(age: Duration) -> Self {
        Cache {
            cache: HashMap::new(),
            age,
        }
    }

    pub fn insert(&mut self, record_type: RecordType, results: Result<Vec<QueryResult>, Errors>) {
        self.cache.insert(record_type, CacheEntry {
            results,
            timestamp: Instant::now()
        });
    }

    pub fn get(&self, record_type: &RecordType) -> Option<Result<Vec<QueryResult>, Errors>> {
        let entry = self.cache.get(record_type)?;
        if entry.timestamp.elapsed() > self.age {
            None
        } else {
            Some(entry.results.clone())
        }
    }

    pub fn cleanup(&mut self) {
        self.cache.retain(|_, entry| entry.timestamp.elapsed() < self.age);
    }
}

