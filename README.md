# Mimalloc Rust

[![Latest Version]][crates.io] [![Documentation]][docs.rs]

A drop-in global allocator wrapper around the [mimalloc](https://github.com/microsoft/mimalloc) allocator.
Mimalloc is a general purpose, performance oriented allocator built by Microsoft.

> **Note:** This is a fork of [`mimalloc`](https://crates.io/crates/mimalloc) /
> [`libmimalloc-sys`](https://crates.io/crates/libmimalloc-sys) published under
> the names **`bc-mimalloc`** and **`bc-libmimalloc-sys`**. It updates the
> bundled mimalloc C library to **v2.3.0** (default) and **v3.3.0** (opt-in via
> the `v3` feature).

## Usage

```rust
use bc_mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

## Requirements

A **C** compiler is required for building [mimalloc](https://github.com/microsoft/mimalloc) with cargo.

## Usage with secure mode

Using secure mode adds guard pages,
randomized allocation, encrypted free lists, etc. The performance penalty is usually
around 10% according to [mimalloc](https://github.com/microsoft/mimalloc)
own benchmarks.

To enable secure mode, put in `Cargo.toml`:

```ini
[dependencies]
bc-mimalloc = { version = "*", features = ["secure"] }
```

## Usage with v3

By default this library uses mimalloc `v2` (currently **v2.3.0**).
To enable `v3` (**v3.3.0**), put in `Cargo.toml`:

```ini
[dependencies]
bc-mimalloc = { version = "*", features = ["v3"] }
```

[crates.io]: https://crates.io/crates/bc-mimalloc
[Latest Version]: https://img.shields.io/crates/v/bc-mimalloc.svg
[Documentation]: https://docs.rs/bc-mimalloc/badge.svg
[docs.rs]: https://docs.rs/bc-mimalloc
