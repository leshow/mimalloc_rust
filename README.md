# Mimalloc Rust

[![Latest Version]][crates.io] [![Documentation]][docs.rs]

A drop-in global allocator wrapper around the [mimalloc](https://github.com/microsoft/mimalloc) allocator.
Mimalloc is a general purpose, performance oriented allocator built by Microsoft.

> **Note:** This is a fork of [`mimalloc`](https://crates.io/crates/mimalloc) /
> [`libmimalloc-sys`](https://crates.io/crates/libmimalloc-sys) published under
> **`bc-mimalloc`** and **`bc-libmimalloc-sys`**. It bundles
> mimalloc **v3.3.0** by DEFAULT (now recommended by upstream) and **v2.3.0**
> (opt-out via `default-features = false`).

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

## Usage with v2

By default this library uses mimalloc `v3` (currently **v3.3.0**), which is
recommended by upstream. To OPT-OUT and use `v2` (**v2.3.0**) instead, disable default
features in `Cargo.toml`:

```ini
[dependencies]
bc-mimalloc = { version = "*", default-features = false }
```

[crates.io]: https://crates.io/crates/bc-mimalloc
[Latest Version]: https://img.shields.io/crates/v/bc-mimalloc.svg
[Documentation]: https://docs.rs/bc-mimalloc/badge.svg
[docs.rs]: https://docs.rs/bc-mimalloc
