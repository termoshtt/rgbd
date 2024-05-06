//! API for "base" database
//!
//! "base" is a sqlite database contains a table "features" as follows:
//!
//! ```sql
//! CREATE TABLE features (
//!   hash UNIQUE NOT NULL base_features_runtime TEXT NOT NULL DEFAULT empty
//!   clauses TEXT NOT NULL DEFAULT empty
//!   variables TEXT NOT NULL DEFAULT empty
//!   bytes TEXT NOT NULL DEFAULT empty
//!   ccs TEXT NOT NULL DEFAULT empty
//!   cls1 TEXT NOT NULL DEFAULT empty
//!   cls2 TEXT NOT NULL DEFAULT empty
//!   cls3 TEXT NOT NULL DEFAULT empty
//!   cls4 TEXT NOT NULL DEFAULT empty
//!   cls5 TEXT NOT NULL DEFAULT empty
//!   cls6 TEXT NOT NULL DEFAULT empty
//!   cls7 TEXT NOT NULL DEFAULT empty
//!   cls8 TEXT NOT NULL DEFAULT empty
//!   cls9 TEXT NOT NULL DEFAULT empty
//!   cls10p TEXT NOT NULL DEFAULT empty
//!   horn TEXT NOT NULL DEFAULT empty
//!   invhorn TEXT NOT NULL DEFAULT empty
//!   positive TEXT NOT NULL DEFAULT empty
//!   negative TEXT NOT NULL DEFAULT empty
//!   hornvars_mean TEXT NOT NULL DEFAULT empty
//!   hornvars_variance TEXT NOT NULL DEFAULT empty
//!   hornvars_min TEXT NOT NULL DEFAULT empty
//!   hornvars_max TEXT NOT NULL DEFAULT empty
//!   hornvars_entropy TEXT NOT NULL DEFAULT empty
//!   invhornvars_mean TEXT NOT NULL DEFAULT empty
//!   invhornvars_variance TEXT NOT NULL DEFAULT empty
//!   invhornvars_min TEXT NOT NULL DEFAULT empty
//!   invhornvars_max TEXT NOT NULL DEFAULT empty
//!   invhornvars_entropy TEXT NOT NULL DEFAULT empty
//!   balancecls_mean TEXT NOT NULL DEFAULT empty
//!   balancecls_variance TEXT NOT NULL DEFAULT empty
//!   balancecls_min TEXT NOT NULL DEFAULT empty
//!   balancecls_max TEXT NOT NULL DEFAULT empty
//!   balancecls_entropy TEXT NOT NULL DEFAULT empty
//!   balancevars_mean TEXT NOT NULL DEFAULT empty
//!   balancevars_variance TEXT NOT NULL DEFAULT empty
//!   balancevars_min TEXT NOT NULL DEFAULT empty
//!   balancevars_max TEXT NOT NULL DEFAULT empty
//!   balancevars_entropy TEXT NOT NULL DEFAULT empty
//!   vcg_vdegree_mean TEXT NOT NULL DEFAULT empty
//!   vcg_vdegree_variance TEXT NOT NULL DEFAULT empty
//!   vcg_vdegree_min TEXT NOT NULL DEFAULT empty
//!   vcg_vdegree_max TEXT NOT NULL DEFAULT empty
//!   vcg_vdegree_entropy TEXT NOT NULL DEFAULT empty
//!   vcg_cdegree_mean TEXT NOT NULL DEFAULT empty
//!   vcg_cdegree_variance TEXT NOT NULL DEFAULT empty
//!   vcg_cdegree_min TEXT NOT NULL DEFAULT empty
//!   vcg_cdegree_max TEXT NOT NULL DEFAULT empty
//!   vcg_cdegree_entropy TEXT NOT NULL DEFAULT empty
//!   vg_degree_mean TEXT NOT NULL DEFAULT empty
//!   vg_degree_variance TEXT NOT NULL DEFAULT empty
//!   vg_degree_min TEXT NOT NULL DEFAULT empty
//!   vg_degree_max TEXT NOT NULL DEFAULT empty
//!   vg_degree_entropy TEXT NOT NULL DEFAULT empty
//!   cg_degree_mean TEXT NOT NULL DEFAULT empty
//!   cg_degree_variance TEXT NOT NULL DEFAULT empty
//!   cg_degree_min TEXT NOT NULL DEFAULT empty
//!   cg_degree_max TEXT NOT NULL DEFAULT empty
//!   cg_degree_entropy TEXT NOT NULL DEFAULT empty
//! )
//! ```

use crate::cache::get_db;
use anyhow::Result;
use std::collections::BTreeMap;

/// Sizes of instance
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size {
    pub variables: usize,
    pub clauses: usize,
    pub bytes: usize,
}

/// Get sizes of instances
///
/// Some instances does not have size information, in that case, the value is `None`.
pub fn get_sizes() -> Result<BTreeMap<String, Option<Size>>> {
    let path = get_db("base")?;
    let conn = rusqlite::Connection::open(path)?;

    let mut stmt = conn.prepare("SELECT hash, variables, clauses, bytes FROM features")?;
    let iter = stmt.query_map([], |row| {
        Ok([
            row.get::<_, String>(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        ])
    })?;
    let mut out = BTreeMap::new();
    for res in iter {
        let [hash, variables, clauses, bytes] = res?;
        let size = if variables == "empty" || clauses == "empty" || bytes == "empty" {
            None
        } else {
            Some(Size {
                variables: variables.parse()?,
                clauses: clauses.parse()?,
                bytes: bytes.parse()?,
            })
        };
        out.insert(hash, size);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sizes() {
        let sizes = get_sizes().unwrap();
        assert_eq!(
            sizes["00213e27dabcf679205144f3dde5d37e"],
            Some(Size {
                clauses: 5279,
                variables: 250,
                bytes: 117579,
            })
        );
    }
}
