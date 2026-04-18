use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;

mod hash_table;
use hash_table::{current_timestamp, HashTable};

enum Command {
    Insert(String, u32, u32),
    Delete(String, u32),
    Update(String, u32, u32),
    Search(String, u32),
    Print(u32),
}
fn main() {
    let file = File::open("commands.txt").expect("Could not open commands.txt");
    let reader = BufReader::new(file);
    let mut commands = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        
        let parts: Vec<&str> = trimmed.split(',').collect();

        match parts.as_slice() {
            ["threads", ..] => continue,
            
            // Format: insert,name,salary,priority
            ["insert", name, salary, priority] => {
                let s: u32 = salary.parse().unwrap();
                let p: u32 = priority.parse().unwrap();
                commands.push(Command::Insert(name.to_string(), s, p));
            }

            // Format: delete,name,placeholder,priority
            ["delete", name, _, priority] => {
                let p: u32 = priority.parse().unwrap();
                commands.push(Command::Delete(name.to_string(), p));
            }

            // Format: update,name,salary,priority
            ["update", name, salary, priority] => {
                let s: u32 = salary.parse().unwrap();
                let p: u32 = priority.parse().unwrap();
                commands.push(Command::Update(name.to_string(), s, p));
            }

            // Format: search,name,placeholder,priority
            ["search", name, _, priority] => {
                let p: u32 = priority.parse().unwrap();
                commands.push(Command::Search(name.to_string(), p));
            }

            // Format: print,placeholder,placeholder,priority
            ["print", _, _, priority] => {
                let p: u32 = priority.parse().unwrap();
                commands.push(Command::Print(p));
            }

            _ => println!("Skipping unrecognized line structure: {}", trimmed),
        }
    }

    let log_file = Arc::new(Mutex::new(
        OpenOptions::new().create(true).write(true).truncate(true).open("hash.log").unwrap()
    ));

    let hash_table = Arc::new(HashTable::new(Arc::clone(&log_file)));
    let mut handles = Vec::new();

    for cmd in commands {
        let ht = Arc::clone(&hash_table);
        let log = Arc::clone(&log_file);

        let handle = thread::spawn(move || {
            let thread_id = match &cmd {
                Command::Insert(_, _, id) | Command::Update(_, _, id) => *id,
                Command::Delete(_, id) | Command::Search(_, id) | Command::Print(id) => *id,
            };

            // Requirement: Log status before work begins
            {
                let mut file = log.lock().unwrap();
                writeln!(file, "{}: THREAD {} WAITING FOR MY TURN", current_timestamp(), thread_id).unwrap();
                writeln!(file, "{}: THREAD {} AWAKENED FOR WORK", current_timestamp(), thread_id).unwrap();
            }

            match cmd {
                Command::Insert(n, s, id) => ht.insert(n, s, id),
                Command::Delete(n, id) => ht.delete(n, id),
                Command::Update(n, s, id) => ht.update(n, s, id),
                Command::Search(n, id) => ht.search(n, id),
                Command::Print(id) => ht.print_table(id),
            }
        });
        handles.push(handle);
    }

    for h in handles { h.join().unwrap(); }

    // Requirement: Final print must occur even if last command was PRINT
    println!("Current Database:");
    hash_table.final_print();
}
