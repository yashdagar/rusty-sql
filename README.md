# Simple DBMS in Rust

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-blue?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A from-scratch implementation of a simple relational database management system (DBMS) in Rust, inspired by [Let's Build a Simple Database](https://cstack.github.io/db_tutorial/).

## Goals

- [ ] **REPL** - Interactive command-line interface
- [ ] **SQL Compiler** - Parses and executes SQL-like queries
- [ ] **B+ Trees** - For effective indexing of tables
- [ ] **Transaction Management** - ACID transactions
- [ ] **Buffer Pool** - Page-based storage with disk I/O
- [ ] **LRU Replacement** - Page eviction policy
- [ ] **Query Parser** - SQL parser with basic query optimization

## Non-Goals

- Multithreaded concurrency control
- Crash recovery mechanisms
- Deadlock detection/prevention

## Related Projects

- [Let's Build a Simple Database](https://cstack.github.io/db_tutorial/) - Primary inspiration
- [sqlite](https://sqlite.org) - Production-grade reference
- [buntdb](https://github.com/tidwall/buntdb) - Embedded key/value store

## Contributing

1. Fork the repo
2. Create your feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -am 'Add my feature'`)
4. Push to branch (`git push origin feature/my-feature`)
5. Open Pull Request

## License

MIT License - see [LICENSE](LICENSE) file.
