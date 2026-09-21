<div align="center">
  <img src="./logo.png" alt="BrachioDB logo" width="320">

  <h1>BrachioDB</h1>

  <p>A small persistent key-value store built to learn database internals.</p>
</div>

> [!NOTE]
> BrachioDB is an early-stage learning project. The storage engine described
> here is still under development.

## About

BrachioDB is a lightweight key-value store written in Rust. The project focuses
on correctness, understandable components, and hands-on exploration of storage
engine fundamentals.

The user-facing operations are deliberately small:

- `GET <key>`
- `SET <key> <value>`
- `DELETE <key>`

Keys and values are treated as opaque byte sequences by the storage engine.

## Architecture

```text
             +------------------+
             |    CLI / REPL    |
             +---------+--------+
                       |
             +---------v--------+
             |     KV engine    |
             +---------+--------+
                       |
          +------------v-------------+
          | WAL / Snapshot / Recovery|
          +--------------------------+
```

Mutations are written to a write-ahead log before being reported as successful.
On startup, BrachioDB restores the latest snapshot and replays subsequent log
records. Checkpoints periodically replace accumulated log history with a new
snapshot.

## Planned usage

```console
$ brachiodb --data-dir <path>
```

The data directory is explicit so separate experiments and tests do not share
state accidentally.

## Development roadmap

- [ ] In-memory key-value engine and REPL
- [ ] Write-ahead log and restart recovery
- [ ] Snapshots and checkpoints
- [ ] Data-directory locking
- [ ] End-to-end and crash-recovery tests

## Building

BrachioDB requires the Rust toolchain:

```console
$ cargo build
$ cargo test
```

## Design

See [the design document](docs/DESIGN.md) for the storage model, durability
rules, recovery behavior, and project boundaries.
