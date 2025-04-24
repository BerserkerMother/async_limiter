//! Simple rate limiter for async tasks.
//!
//! Provides a structure to control rate of something. Something can be API calls to external
//! resource or anything you want to be limited to requests per interval.
//!
//! # Examples:
//! ```
//! // allows 1000 requests every 47 seconds to rick and morty api!
//! let limiter = RateLimiter::new(1000, Duration::from_secs(47));
//! for _ in 0..10000 {
//!     let limiter = limiter.clone();
//!     tokio::spawn(async move {
//!         limiter.wait().await;
//!         let body = reqwest::get("https://rickandmortyapi.com/api/episode/16")
//!             .await
//!             .unwrap()
//!             .text()
//!             .await
//!             .unwrap();
//!     });
//! }
//!```

mod limiter;

pub use limiter::{RateLimiter, Waiter};

async fn example() {}
