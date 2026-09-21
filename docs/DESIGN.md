# BrachioDB Design

## 1. Purpose

BrachioDB is a lightweight persistent key-value store built to explore how a
storage engine works. The project favors correctness, understandable code, and
small components over production-level performance or a broad feature set.

This document defines the contracts that keep the storage components compatible.
Implementation details may evolve as long as those contracts remain intact.

## 2. Scope

The store exposes three operations:

- `GET` reads the value associated with a key.
- `SET` creates or replaces a key-value pair.
- `DELETE` removes a key-value pair.

Keys and values are opaque byte sequences. The storage layer does not interpret
their contents or impose an application-level structure.

The MVP is complete when successful mutations survive a normal restart and the
store can recover to a valid state after an interrupted write.

## 3. System Shape

BrachioDB is divided into small layers:

1. The CLI opens a data directory and starts the REPL.
2. The KV engine applies `GET`, `SET`, and `DELETE` operations.
3. The persistence layer owns the write-ahead log, snapshots, checkpoints, and
   startup recovery.

A typical invocation has this shape:

```text
brachiodb --data-dir <path>
```

Only one BrachioDB process may open a data directory at a time. The initial
implementation is single-threaded and executes operations serially.

## 4. Durability

`SET` and `DELETE` records must be durably appended to the write-ahead log
before the corresponding in-memory mutation is reported as successful. `GET`
does not modify state and is not logged.

Acknowledged operations must remain visible after a normal restart. If durable
logging fails, the operation must return an error rather than modifying the
visible state and reporting success.

The on-disk format must include a format identifier and version. Records must
be framed so recovery can distinguish a complete record from an incomplete
trailing write. Checksums or equivalent validation must make corruption
detectable instead of silently accepting damaged data.

## 5. Recovery and Checkpoints

On startup, the engine:

1. validates and loads the latest complete snapshot, if one exists;
2. replays valid WAL records written after that snapshot;
3. ignores an incomplete trailing record caused by an interrupted append; and
4. reports corruption or an unsupported format when encountered elsewhere.

A checkpoint writes the current state to a temporary snapshot, makes that
snapshot durable, and atomically publishes it. Only after the new snapshot is
durable may WAL history covered by the checkpoint be discarded.

Recovery and checkpoint operations must be idempotent: repeating them after an
interruption must not produce a different logical state.

## 6. Errors and Limits

Malformed files, unsupported format versions, checksum failures, and I/O
failures are explicit errors. Configured size limits for keys, values, records,
and snapshots must be checked before allocating memory based on on-disk data.

The storage engine must not interpret a corrupt length field as permission to
allocate unbounded memory.

## 7. Non-goals for the MVP

The following features are intentionally deferred:

- multi-operation transactions;
- secondary indexes;
- concurrent clients or processes;
- a network protocol and server;
- replication and distributed consensus;
- production-level performance guarantees.

## 8. Implementation Path

Development proceeds in small working slices:

1. Build an in-memory KV engine and REPL.
2. Define and test the WAL record codec.
3. Add durable WAL append and restart recovery.
4. Add snapshots and explicit checkpoints.
5. Enforce exclusive ownership of the data directory.
6. Add end-to-end and crash-recovery tests.

Each stage should preserve the contracts established by earlier stages and
leave the project in a testable state.
