mod hash_table;
mod logger;

use hash_table::ConcurrentTable;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use std::thread;

fn main() {
    let table = Arc::new(ConcurrentTable::new());
    let file = File::open("commands.txt").expect("Unable to open commands.txt");
    let reader = BufReader::new(file);
    let mut handles = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let table_ref = Arc::clone(&table);

        let handle = thread::spawn(move || {
            let parts: Vec<&str> = line.split(',').collect();
            let cmd = parts[0];
            
            // Assume the last part is always priority, except for update which has salary then priority? 
            // Let's parse based on command types.
            let priority: i32 = parts.last().unwrap().parse().unwrap_or(0);

            logger::log(&format!("THREAD {} WAITING FOR MY TURN", priority));
            table_ref.wait_for_turn(priority);
            logger::log(&format!("THREAD {} AWAKENED FOR WORK", priority));
            logger::log(&format!("THREAD {}, {}", priority, line));

            match cmd {
                "insert" => {
                    let name = parts[1].to_string();
                    let salary: u32 = parts[2].parse().unwrap();
                    logger::log(&format!("THREAD {} WRITE LOCK ACQUIRED", priority));
                    match table_ref.insert(name.clone(), salary) {
                        Ok(_) => println!("INSERT*Inserted {},{}", name, salary),
                        Err(h) => println!("INSERT*Insert failed. Entry {} is a duplicate.", h),
                    }
                    logger::log(&format!("THREAD {} WRITE LOCK RELEASED", priority));
                }
                "update" => {
                    let name = parts[1];
                    let salary: u32 = parts[2].parse().unwrap();
                    logger::log(&format!("THREAD {} WRITE LOCK ACQUIRED", priority));
                    match table_ref.update(name, salary) {
                        Ok((h, old)) => println!("UPDATE*Updated record {} from {} to {}", h, old, salary),
                        Err(h) => println!("UPDATE*Update failed. Entry {} not found.", h),
                    }
                    logger::log(&format!("THREAD {} WRITE LOCK RELEASED", priority));
                }
                "delete" => {
                    let name = parts[1];
                    logger::log(&format!("THREAD {} WRITE LOCK ACQUIRED", priority));
                    match table_ref.delete(name) {
                        Ok(_) => println!("DELETE*Deleted record for {}", name),
                        Err(h) => println!("DELETE*Entry {} not deleted. Not in database.", h),
                    }
                    logger::log(&format!("THREAD {} WRITE LOCK RELEASED", priority));
                }
                "search" => {
                    let name = parts[1];
                    logger::log(&format!("THREAD {} READ LOCK ACQUIRED", priority));
                    match table_ref.search(name) {
                        Some((_, s)) => println!("SEARCH*Found: {},{}", name, s),
                        None => println!("SEARCH*Not Found: {} not found.", name),
                    }
                    logger::log(&format!("THREAD {} READ LOCK RELEASED", priority));
                }
                "print" => {
                    print_table(&table_ref);
                }
                _ => {}
            }

            table_ref.next_turn();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Final Print
    println!("\nFinal Printout:");
    print_table(&table);
}

fn print_table(table: &ConcurrentTable) {
    let records = table.get_all_sorted();
    println!("PRINT*Current Database:");
    for (h, n, s) in records {
        println!("{},{},{}", h, n, s);
    }
}
