# kv-rs

A simple in-memory key-value store written in Rust. Persists data via an append-only log (`wal.log`).

## Running

Start the server:
```bash
cargo run --bin kvs
```

Server listens on `127.0.0.1:7878`.

## Client

One-shot commands:
```bash
kvc set <key> <value>
kvc get <key>
kvc del <key>
kvc list
```

Interactive REPL mode:
```bash
kvc -i
```
```
Commands: get, set, list, del.
 q to quit.
> set foo bar
inserted: foo
> get foo
bar
> q
```

You can also connect directly with `nc`:
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

- [+] Persistence (append-only log)
- [+] Dedicated CLI client
- [+] Error responses sent back to client
- [+] Case-preserving keys and values
