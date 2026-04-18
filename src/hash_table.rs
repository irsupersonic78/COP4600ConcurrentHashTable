use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex, RwLock};

pub fn current_timestamp() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros()
}

pub fn jenkins_hash(key: &str) -> u32 {
    let mut hash: u32 = 0;
    for b in key.bytes() {
        hash = hash.wrapping_add(b as u32);
        hash = hash.wrapping_add(hash << 10);
        hash ^= hash >> 6;
    }
    hash = hash.wrapping_add(hash << 3);
    hash ^= hash >> 11;
    hash = hash.wrapping_add(hash << 15);
    hash
}

pub struct HashRecord {
    pub hash: u32,
    pub name: String,
    pub salary: u32,
    pub next: Option<Box<HashRecord>>,
}

pub struct HashTable {
    data: RwLock<Option<Box<HashRecord>>>,
    log: Arc<Mutex<File>>,
}

impl HashTable {
    pub fn new(log_file: Arc<Mutex<File>>) -> Self {
        HashTable {
            data: RwLock::new(None),
            log: log_file,
        }
    }

    fn write_log(&self, msg: &str) {
        let mut file = self.log.lock().unwrap();
        writeln!(file, "{}", msg).unwrap();
    }

    pub fn insert(&self, name: String, salary: u32, thread_id: u32) {
        let hash = jenkins_hash(&name);
        self.write_log(&format!("{}: THREAD {},INSERT,{},{}", current_timestamp(), thread_id, name, salary));

        let mut list = self.data.write().unwrap();
        self.write_log(&format!("{}: THREAD {} WRITE LOCK ACQUIRED", current_timestamp(), thread_id));

        let mut curr = &mut *list;

        loop {
            let should_break = match curr {
                Some(node) if node.hash == hash => {
                    println!("Insert failed. Entry {} is a duplicate.", hash);
                    self.write_log(&format!("{}: THREAD {} WRITE LOCK RELEASED", current_timestamp(), thread_id));
                    return;
                }
                Some(node) if node.hash < hash => false,
                _ => true,
            };

            if should_break { break; }
            curr = &mut curr.as_mut().unwrap().next;
        }

        let new_node = Box::new(HashRecord {
            hash,
            name: name.clone(),
            salary,
            next: curr.take(),
        });
        *curr = Some(new_node);

        println!("Inserted {},{},{}", hash, name, salary);
        self.write_log(&format!("{}: THREAD {} WRITE LOCK RELEASED", current_timestamp(), thread_id));
    }
    // In hash_table.rs

    pub fn delete(&self, name: String, thread_id: u32) {
        let hash = jenkins_hash(&name);
        self.write_log(&format!("{}: THREAD {},DELETE,{},0", current_timestamp(), thread_id, name));

        let mut list = self.data.write().unwrap();
        self.write_log(&format!("{}: THREAD {} WRITE LOCK ACQUIRED", current_timestamp(), thread_id));

        let mut curr = &mut *list;
        let mut found = false;

        loop {
            let action = match curr {
                Some(node) if node.hash == hash => 1, // Found
                Some(node) if node.hash < hash => 2, // Keep looking
                _ => 3, // Not in list (since it's sorted)
            };

            if action == 1 {
                let node = curr.take().unwrap();
                println!("Deleted record for {},{},{}", hash, name, node.salary);
                *curr = node.next;
                found = true;
                break;
            } else if action == 3 {
                break;
            }
            curr = &mut curr.as_mut().unwrap().next;
        }

        if !found {
            // Correct string for instructions: "Entry <hash> not deleted. Not in database."
            println!("Entry {} not deleted. Not in database.", hash);
        }
        self.write_log(&format!("{}: THREAD {} WRITE LOCK RELEASED", current_timestamp(), thread_id));
    }

    pub fn search(&self, name: String, thread_id: u32) {
        let hash = jenkins_hash(&name);
        self.write_log(&format!("{}: THREAD {},SEARCH,{},0", current_timestamp(), thread_id, name));

        let list = self.data.read().unwrap();
        self.write_log(&format!("{}: THREAD {} READ LOCK ACQUIRED", current_timestamp(), thread_id));

        let mut curr = list.as_ref();
        let mut found = false;

        while let Some(node) = curr {
            if node.hash == hash {
                println!("Found: {},{},{}", hash, name, node.salary);
                found = true;
                break;
            }
            curr = node.next.as_ref();
        }

        if !found {
            // Correct string for instructions: "Not Found: <name> not found."
            println!("Not Found: {} not found.", name);
        }
        self.write_log(&format!("{}: THREAD {} READ LOCK RELEASED", current_timestamp(), thread_id));
    }

    pub fn update(&self, name: String, new_salary: u32, thread_id: u32) {
        let hash = jenkins_hash(&name);
        self.write_log(&format!("{}: THREAD {},UPDATE,{},{}", current_timestamp(), thread_id, name, new_salary));

        let mut list = self.data.write().unwrap();
        self.write_log(&format!("{}: THREAD {} WRITE LOCK ACQUIRED", current_timestamp(), thread_id));

        let mut curr = list.as_mut();
        let mut found = false;

        while let Some(node) = curr {
            if node.hash == hash {
                let old_sal = node.salary;
                node.salary = new_salary;
                println!("Updated record {} from {},{},{} to {},{},{}", hash, hash, name, old_sal, hash, name, new_salary);
                found = true;
                break;
            }
            curr = node.next.as_mut();
        }

        if !found {
            println!("Update failed. Entry {} not found.", hash);
        }
        self.write_log(&format!("{}: THREAD {} WRITE LOCK RELEASED", current_timestamp(), thread_id));
    }

    pub fn print_table(&self, thread_id: u32) {
        self.write_log(&format!("{}: THREAD {},PRINT,0,0", current_timestamp(), thread_id));
        let list = self.data.read().unwrap();
        self.write_log(&format!("{}: THREAD {} READ LOCK ACQUIRED", current_timestamp(), thread_id));

        println!("Current Database:");
        let mut curr = list.as_ref();
        while let Some(node) = curr {
            println!("{},{},{}", node.hash, node.name, node.salary);
            curr = node.next.as_ref();
        }
        self.write_log(&format!("{}: THREAD {} READ LOCK RELEASED", current_timestamp(), thread_id));
    }

    pub fn final_print(&self) {
        let list = self.data.read().unwrap();
        let mut curr = list.as_ref();
        while let Some(node) = curr {
            println!("{},{},{}", node.hash, node.name, node.salary);
            curr = node.next.as_ref();
        }
    }
}
