# mini-redis

## Group Name

Ok(Group)

## Group Members

| Name | NetID |
| --- | --- |
| Avery Weng | zweng9 |
| Ruotian Yang | alberty5 |
| Houlin Huang | houlinh2 |

## Project Introduction

This is a small Redis-style in-memory key-value store written in Rust. 

It consists of a TCP server that stores data and a command-line client that sends text commands such as `SET name`, `GET name`, and `DEL name`. Multiple clients can connect to the server at the same time and share the same data.

### Goals and Objectives

- A key-value server that supports `SET`, `GET`, and `DEL`.
- Support multiple concurrent clients safely.
- Handle invalid input gracefully with clear error messages.
- Keep the code organized.

### Why This Project

Redis is a widely used tool in industry, and a simplified version is suitable for Rust, as server that many clients access concurrently resonates with Rust's features, such as enums, ownership, structs, traits, generics, and data structures. 

## Technical Overview

```
src/
├── lib.rs          // module declarations
├── command.rs      // command parsing
├── storage.rs      // storage trait and implementations
└── bin/
    ├── server.rs   // TCP server
    └── client.rs   // command-line client
```

**`command.rs`**
Turns a line of text from a client into a `Command` enum (for example `Get`, `Set`, `Del`). Invalid input (empty lines, unknown commands, wrong number of arguments) is reported through a custom error enum and `Result`, so the server can reply with an error instead of panicking.

**`storage.rs`**
A `Storage` trait defines the operations the server needs (get, set, delete). We plan two implementations:
- an in-memory store backed by a `HashMap`
- a persistent store that also saves data to a file and reloads it on startup

**`bin/server.rs`**
Listens on a TCP port with `std::net::TcpListener` and spawns a thread for each client connection. The store is shared between threads using `Arc<Mutex<...>>`. Each thread reads commands line by line, executes them against the store, and writes responses back to the client.

**`bin/client.rs`**
Connects to the server, reads commands typed by the user, sends them over the connection, and prints the server's responses.

## Timeline

### Checkpoint 1 (11/09 – 11/13)

* Define the `Command` enum and `Storage` trait (with room for TTL).
* Implement the command parser (including the `EX` option) with unit tests.
* Implement the in-memory store with tests.
* Build a local terminal version so `SET`/`GET`/`DEL` work end to end.

### Checkpoint 2 (11/30 – 12/04)

* TCP server and command-line client (single client first).
* Multithreading with `Arc<Mutex<...>>` so many clients can share data; integration tests with multiple concurrent clients.
* Key expiration (TTL).
* File persistence as a second `Storage` implementation; make the server generic over the `Storage` trait.

### Final Submission (12/07)

* Clean up code, pass `cargo clippy`, expand tests, and finish documentation.
* If time allows: background expiry cleanup, more data types, pub/sub, or RESP compatibility.

## Possible Challenges

- TCP (`std::net`), threads, `Arc`, and `Mutex` were not covered in our homework, so we will need to learn them.
- Sharing mutable state between threads requires understanding `Send`, `Sync`, and lock lifetimes.
- The `Storage` trait needs to work for both in-memory and file-backed implementations.
- Saving to disk without corrupting data and removing expired keys at the right time.
- Writing reliable tests for behavior that involves multiple clients and connections.
- Using branches and pull requests to avoid conflicts when we work in parallel.

## References

- [Redis documentation](https://redis.io/docs/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Tokio's mini-redis tutorial](https://tokio.rs/tokio/tutorial)
