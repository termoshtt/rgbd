//! API for "meta" database
//!
//! "meta" is a sqlite database contains a table "features" as follows:
//!
//! ```sql
//! CREATE TABLE features (
//!   hash UNIQUE NOT NULL
//!   local TEXT NOT NULL DEFAULT None
//!   filename TEXT NOT NULL DEFAULT None
//!   isohash TEXT NOT NULL DEFAULT empty
//!   family TEXT NOT NULL DEFAULT empty
//!   author TEXT NOT NULL DEFAULT empty
//!   track TEXT NOT NULL DEFAULT None
//!   result TEXT NOT NULL DEFAULT unknown
//!   proceedings TEXT NOT NULL DEFAULT empty
//!   minisat1m TEXT NOT NULL DEFAULT empty
//! )
//! ```

use crate::{
    cache::{cache_dir, get_db},
    Digest, BASE_URL,
};
use anyhow::{Context, Result};
use std::{collections::BTreeMap, fs};
use url::Url;

/// Whether the instance is satisfiable or not
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SatResult {
    Sat,
    UnSat,
    Unknown,
}

/// Get the results of instances
pub fn get_results() -> Result<BTreeMap<String, SatResult>> {
    let path = get_db("meta")?;
    let conn = rusqlite::Connection::open(path)?;

    let mut stmt = conn.prepare("SELECT hash, result FROM features")?;
    let iter = stmt.query_map([], |row| Ok([row.get::<_, String>(0)?, row.get(1)?]))?;
    let mut out = BTreeMap::new();
    for res in iter {
        let [hash, result] = res?;
        let result = match result.as_str() {
            "sat" => SatResult::Sat,
            "unsat" => SatResult::UnSat,
            _ => SatResult::Unknown,
        };
        out.insert(hash, result);
    }
    Ok(out)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_results() {
        let results = get_results().unwrap();
        assert_eq!(results["00213e27dabcf679205144f3dde5d37e"], SatResult::Sat);
        assert_eq!(
            results["0020aa0c69379226948904ad455b6c09"],
            SatResult::UnSat
        );
        assert_eq!(
            results["00072cf107ae1349c8c59a15c5ce4af1"],
            SatResult::Unknown
        );
    }
}
