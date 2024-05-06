//! Unofficial Rust client for [GBD Benchmark Database](https://github.com/Udopia/gbd), [A Problem Meta-Data Library for Research in SAT](https://easychair.org/publications/paper/jQXv)
//!
//! This crate downloads instance lists and actual benchmark instances from <https://benchmark-database.de/> on-demand, and caches them locally.
//!
//! Examples
//! ---------
//!
//! Get instances of the main track in [SAT Competition 2023](https://satcompetition.github.io/2023/)
//!
//! ```rust
//! use rgbd::get_track;
//!
//! let instances = get_track("main_2023").unwrap();
//! assert_eq!(instances.len(), 400);
//!
//! // Take some small instance
//! let cnf = instances[14].read().unwrap();
//! assert_eq!(cnf.num_variables, 45);
//! assert_eq!(cnf.num_clauses, 376);
//! ```

mod base;
mod cache;
mod digest;
mod meta;
mod parse;

pub use base::*;
pub use digest::*;
pub use meta::*;
pub use parse::*;

const BASE_URL: &str = "https://benchmark-database.de";
