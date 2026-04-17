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

The table uses:

    BTreeMap<u32, Record>

- Maintains sorted order by hash
- Simplifies printing
- Avoids manual linked list and pointer management

---

## Memory Safety

Rust uses **ownership** instead of manual memory management:
- No `malloc` or `free`
- Memory is automatically cleaned up
- Prevents dangling pointers and double frees

---

## Concurrency

Shared state will use:

    Arc<RwLock<HashTable>>

- `Arc` → shared ownership across threads  
- `RwLock` → multiple readers or one writer  

Mapping:
- search → read lock  
- insert/delete/update → write lock  

---

## Key Differences from C

- No manual memory management  
- No unsafe pointers  
- Built-in thread safety with `Arc` and `RwLock`  
- Cleaner implementation using standard library data structures  
