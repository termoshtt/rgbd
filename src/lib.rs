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

pub mod base;

mod digest;
mod parse;

pub use digest::*;
pub use parse::*;

use anyhow::{Context, Result};
use std::{fs, path::PathBuf};
use url::Url;

const BASE_URL: &str = "https://benchmark-database.de";

/// The root directory where the database is cached
pub fn cache_dir() -> PathBuf {
    directories::ProjectDirs::from("", "", "rgbd")
        .unwrap()
        .cache_dir()
        .to_owned()
}

/// Get a list of instances for a given track
pub fn get_track(track: &str) -> Result<Vec<Digest>> {
    let cache = cache_dir().join("tracks").join(track);
    let response = if cache.exists() {
        fs::read_to_string(&cache)?
    } else {
        let req = ureq::get(&format!("{BASE_URL}/getinstances"))
            .query("query", &format!("track={}", track))
            .query("context", "cnf");
        log::info!("GET {}", req.url());
        let response = req.call()?.into_string()?;
        fs::create_dir_all(cache.parent().unwrap())?;
        fs::write(&cache, &response)?;
        response
    };
    let urls = response
        .lines()
        .map(|line| {
            Url::parse(line).with_context(|| format!("Track contains invalid URL: {}", line))
        })
        .collect::<Result<Vec<Url>>>()?;
    urls.into_iter().map(|url| Digest::from_url(&url)).collect()
}
