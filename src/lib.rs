//! Unofficial Rust client for [GBD Benchmark Database](https://github.com/Udopia/gbd), [A Problem Meta-Data Library for Research in SAT](https://easychair.org/publications/paper/jQXv)
//!
//! This crate downloads instance lists and actual benchmark instances from <https://benchmark-database.de/> on-demand, and caches them locally.
//!

use anyhow::{Context, Result};
use std::path::PathBuf;
use url::Url;

const BASE_URL: &str = "https://benchmark-database.de/";

/// The root directory where the database is cached
pub fn cache_dir() -> PathBuf {
    directories::ProjectDirs::from("", "", "rgbd")
        .unwrap()
        .cache_dir()
        .to_owned()
}

/// Digest (hash value) of a benchmark instance
pub struct Digest(String);

impl Digest {
    pub fn as_file_url(&self) -> Url {
        Url::parse(&format!("{BASE_URL}/file/{}", self.0)).unwrap()
    }

    pub fn from_url(url: &Url) -> Result<Self> {
        let digest = url
            .path_segments()
            .context("URL is cannot be a base")?
            .last()
            .context("URL does not have path")?
            .to_string();
        Ok(Self(digest))
    }
}

/// Get a list of instances for a given track
pub fn get_track(track: &str) -> Result<Vec<Digest>> {
    let response = ureq::get(BASE_URL)
        .query("track", track)
        .query("context", "cnf")
        .call()?;
    let urls = response
        .into_string()?
        .lines()
        .map(|line| Url::parse(line))
        .collect::<Result<Vec<Url>, _>>()?;
    urls.into_iter().map(|url| Digest::from_url(&url)).collect()
}
