use std::collections::LinkedList;
use std::sync::{Arc, Mutex, Condvar};

pub struct HashRecord {
    pub hash: u32,
    pub name: String,
    pub salary: u32,
}

struct TableState {
    records: Vec<HashRecord>,
    current_priority: i32, // Track whose turn it is
}

pub struct ConcurrentTable {
    state: Mutex<TableState>,
    cv: Condvar,
}

impl ConcurrentTable {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(TableState {
                records: Vec::new(),
                current_priority: 0,
            }),
            cv: Condvar::new(),
        }
    }

    pub fn jenkins_hash(key: &str) -> u32 {
        let mut hash: u32 = 0;
        for b in key.as_bytes() {
            hash = hash.wrapping_add(*b as u32);
            hash = hash.wrapping_add(hash << 10);
            hash ^= hash >> 6;
        }
        hash = hash.wrapping_add(hash << 3);
        hash ^= hash >> 11;
        hash = hash.wrapping_add(hash << 15);
        hash
    }

    // Priority Coordination
    pub fn wait_for_turn(&self, priority: i32) {
        let mut state = self.state.lock().unwrap();
        while state.current_priority != priority {
            state = self.cv.wait(state).unwrap();
        }
    }

    pub fn next_turn(&self) {
        let mut state = self.state.lock().unwrap();
        state.current_priority += 1;
        self.cv.notify_all();
    }

    pub fn insert(&self, name: String, salary: u32) -> Result<u32, u32> {
        let hash = Self::jenkins_hash(&name);
        let mut state = self.state.lock().unwrap();
        
        if state.records.iter().any(|r| r.hash == hash) {
            return Err(hash);
        }

        state.records.push(HashRecord { hash, name, salary });
        Ok(hash)
    }

    pub fn search(&self, name: &str) -> Option<(u32, u32)> {
        let hash = Self::jenkins_hash(name);
        let state = self.state.lock().unwrap();
        state.records.iter()
            .find(|r| r.name == name)
            .map(|r| (r.hash, r.salary))
    }

    pub fn update(&self, name: &str, new_salary: u32) -> Result<(u32, u32), u32> {
        let hash = Self::jenkins_hash(name);
        let mut state = self.state.lock().unwrap();
        if let Some(record) = state.records.iter_mut().find(|r| r.name == name) {
            let old = record.salary;
            record.salary = new_salary;
            return Ok((hash, old));
        }
        Err(hash)
    }

    pub fn delete(&self, name: &str) -> Result<u32, u32> {
        let hash = Self::jenkins_hash(name);
        let mut state = self.state.lock().unwrap();
        let len_before = state.records.len();
        state.records.retain(|r| r.name != name);
        
        if state.records.len() < len_before { Ok(hash) } else { Err(hash) }
    }

    pub fn get_all_sorted(&self) -> Vec<(u32, String, u32)> {
        let state = self.state.lock().unwrap();
        let mut sorted = state.records.iter()
            .map(|r| (r.hash, r.name.clone(), r.salary))
            .collect::<Vec<_>>();
        sorted.sort_by_key(|r| r.0);
        sorted
    }
}
