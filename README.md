# async_limiter

[![Crates.io](https://img.shields.io/crates/v/async_limiter)](https://crates.io/crates/async_limiter)
[![Documentation](https://docs.rs/async_limiter/badge.svg)](https://docs.rs/async_limiter)
[![License](https://img.shields.io/crates/l/async_limiter)](https://github.com/udoprog/async_limiter/blob/main/LICENSE)

An asynchronous rate limiter for Rust based on the token bucket algorithm.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
async_limiter = "0.1"
```

## Basic Usage
```rust
let limiter = RateLimiter::new(1000, Duration::from_secs(47));
for _ in 0..10000 {
    let limiter = limiter.clone();
    tokio::spawn(async move {
        limiter.wait().await;
        let body = reqwest::get("https://rickandmortyapi.com/api/episode/16")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    });
}
```

## Contribution
I developed this simple package for my own use cases. I thought it may benefit others. If I missed something or you need a new feature, feel free to open an issue.
