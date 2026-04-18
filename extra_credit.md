# Concurrent Hash Table in Rust

## Overview

This project implements a concurrent hash table in Rust with support for:
- insert
- delete
- search
- update
- print

Each record contains a hash (`u32`), name (`String`), and salary (`u32`).

---

## Design

Design
The table is implemented as a sorted linked list:

Rust
pub struct HashRecord {
    pub hash: u32,
    pub name: String,
    pub salary: u32,
    pub next: Option<Box<HashRecord>>,
}

- Sorted Order: Records are maintained in ascending order by their Jenkins hash value.
- Recursive Structure: Uses Option<Box<HashRecord>> to safely handle the "next" pointer, ensuring memory is owned and automatically managed.
- Efficiency: Sorting allows for early termination during delete and insert operations if the target hash is bypassed.

---

## Memory Safety

Rust uses **ownership** instead of manual memory management:
- No `malloc` or `free`
- Memory is automatically cleaned up
- Prevents dangling pointers and double frees

---

## Concurrency

Concurrency
The system uses a thread-safe wrapper to manage shared state across multiple threads:

Rust
pub struct HashTable {
    data: RwLock<Option<Box<HashRecord>>>,
    log: Arc<Mutex<File>>,
}

- Arc (Atomic Reference Counting): Used in main.rs to share the HashTable and File across multiple thread handles.
- RwLock (Read-Write Lock): - Multiple Readers: search and print_table acquire a read lock, allowing concurrent access.
- Single Writer: insert, delete, and update acquire a write lock, ensuring exclusive access to the linked list.
- Mutex (Mutual Exclusion): Used specifically for the shared log file to ensure that thread-specific log entries do not interleave or corrupt the output.

---

## Key Differences from C

Key Differences from C
- Thread-Safe by Default: The compiler prevents you from sharing the HashTable without a synchronization primitive like RwLock.
- Automatic Cleanup: No need for a destroy_table function; memory is reclaimed as soon as the Arc reference count hits zero.
- Safe Iteration: Using while let and match to traverse the list prevents segmentation faults common in C linked list traversals.
