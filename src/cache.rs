use crate::BASE_URL;
use anyhow::Result;
use std::{fs, io, path::PathBuf};

/// The root directory where the database is cached
pub fn cache_dir() -> PathBuf {
    directories::ProjectDirs::from("", "", "rgbd")
        .unwrap()
        .cache_dir()
        .to_owned()
}

/// Get the path where the database is cached
///
/// If the database does not exist, it will be downloaded from the server.
pub fn get_db(name: &str) -> Result<PathBuf> {
    let db = cache_dir().join("db").join(format!("{name}.db"));
    if !db.exists() {
        fs::create_dir_all(db.parent().unwrap())?;
        let req = ureq::get(format!("{BASE_URL}/getdatabase/{name}").as_str());
        log::info!("GET {}", req.url());
        let response = req.call()?;
        let mut f = fs::File::create(&db)?;
        io::copy(&mut response.into_reader(), &mut f)?;
    }
    Ok(db)
}
