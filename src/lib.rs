//! Unofficial Rust client for [GBD Benchmark Database](https://github.com/Udopia/gbd), [A Problem Meta-Data Library for Research in SAT](https://easychair.org/publications/paper/jQXv)
//!
//! This crate downloads instance lists and actual benchmark instances from <https://benchmark-database.de/> on-demand, and caches them locally.
//!

use anyhow::{Context, Result};
use std::{fs, path::PathBuf};
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
pub fn get_track(track: &str, always_retrieve: bool) -> Result<Vec<Digest>> {
    let cache = cache_dir().join("tracks").join(track);
    let response = if !always_retrieve && cache.exists() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_track_main_2023() {
        let track = "main_2023";
        let instances = get_track(track, true).unwrap();
        assert_eq!(instances.len(), 400);

        // Test cache
        let instances = get_track(track, false).unwrap();
        assert_eq!(instances.len(), 400);
    }
}
