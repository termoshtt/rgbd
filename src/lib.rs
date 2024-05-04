//! Unofficial Rust client for [GBD Benchmark Database](https://github.com/Udopia/gbd), [A Problem Meta-Data Library for Research in SAT](https://easychair.org/publications/paper/jQXv)
//!
//! This crate downloads instance lists and actual benchmark instances from <https://benchmark-database.de/> on-demand, and caches them locally.
//!

use anyhow::Result;
use std::path::PathBuf;
use url::Url;

/// The root directory where the database is cached
pub fn cache_dir() -> PathBuf {
    directories::ProjectDirs::from("", "", "rgbd")
        .unwrap()
        .cache_dir()
        .to_owned()
}

pub struct Digest(String);

impl Digest {
    pub fn as_file_url(&self) -> Url {
        Url::parse(&format!("https://benchmark-database.de/file/{}", self.0)).unwrap()
    }
}

pub fn all_instances() -> Result<Vec<Digest>> {
    todo!()
}

/// Get a list of instances for a given track
pub fn get_track(track: &str) -> Result<Vec<Digest>> {
    todo!()
}
