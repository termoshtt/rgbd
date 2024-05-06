use crate::{cache_dir, BASE_URL, CNF};
use anyhow::{Context, Result};
use std::fs;
use url::Url;

/// Digest (hash value) of a benchmark instance
///
/// Examples
/// ---------
///
/// Mutually convert a digest to a file URL
///
/// ```rust
/// use rgbd::Digest;
///
/// let digest = Digest::new("0123456789abcdef".to_string());
/// let url = digest.as_file_url();
/// assert_eq!(url.as_str(), "https://benchmark-database.de/file/0123456789abcdef");
///
/// let new_digest = Digest::from_url(&url).unwrap();
/// assert_eq!(new_digest, digest);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Digest(String);

impl Digest {
    pub fn new(digest: String) -> Self {
        Self(digest)
    }

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

    fn read_bytes(&self) -> Result<Vec<u8>> {
        let cache = cache_dir().join("file").join(&self.0);
        if cache.exists() {
            Ok(fs::read(&cache)?)
        } else {
            let req = ureq::get(self.as_file_url().as_ref());
            log::info!("GET {}", req.url());
            let mut response = req.call()?.into_reader();
            let mut buf = Vec::new();
            response.read_to_end(&mut buf)?;
            fs::create_dir_all(cache.parent().unwrap())?;
            fs::write(&cache, &buf)?;
            Ok(buf)
        }
    }

    pub fn read(&self) -> Result<CNF> {
        let bytes = self.read_bytes()?;
        let decoder = xz2::read::XzDecoder::new(&bytes[..]);
        CNF::from_dimacs_format(decoder).context("Failed to parse CNF")
    }
}
