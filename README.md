## rkill_lib (forked from reflog/rkill)

[![](https://img.shields.io/crates/v/rkill_lib.svg)](https://crates.io/crates/rkill_lib)

`rkill_lib` helps you kill any processes by PID/name/port number.

### config

```toml
[dependencies]
rkill_lib = "0.1.3"
```

### Usage

```rust
// kill a process by name/PID
rkill_lib::kill_process_by_pid("nginx".to_string());
```
