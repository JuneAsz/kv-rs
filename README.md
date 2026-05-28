# kv-rs

A simple in-memory key-value store server written in Rust. Work in progress.

## Running

```bash
cargo run
```

Server listens on `127.0.0.1:7878`. Connect with `nc` or `telnet`:

```bash
nc 127.0.0.1 7878
```

## Commands

```
SET <key> <value>
GET <key>
DEL <key>
LIST
```

## Roadmap

- [ ] Persistence (append-only log)
- [ ] Dedicated CLI client
- [ ] Error responses sent back to client (currently kills the handler thread)
- [ ] Case-preserving values
