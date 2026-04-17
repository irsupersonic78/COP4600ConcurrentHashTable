// Test module

mod hash_table;

use hash_table::HashTable;

fn main() {
    let mut table = HashTable::new();

    table.insert("Alice", 50000, 100).unwrap();
    table.insert("Bob", 60000, 200).unwrap();
    table.insert("Charlie", 70000, 300).unwrap();

    table.print_all();

    if let Some(record) = table.search("Bob", 200) {
        println!("Found: {},{},{}", record.hash, record.name, record.salary);
    }

    let old = table.update_salary("Bob", 200, 65000).unwrap();
    println!("Old Bob salary: {}", old);

    let deleted = table.delete("Alice", 100);
    println!("Deleted Alice? {}", deleted.is_some());

    table.print_all();
}
