use anyhow::{bail, Context, Result};
use std::io::{self, BufRead};

/// CNF (Conjunctive Normal Form) representation
///
/// ```rust
/// use rgbd::CNF;
///
/// let cnf = CNF::from_dimacs_format_str(r#"
/// p cnf 5 3
/// 1 -5 4 0
/// -1 5 3 4 0
/// -3 -4 0
/// "#).unwrap();
///
/// assert_eq!(cnf.num_variables, 5);
/// assert_eq!(cnf.num_clauses, 3);
/// assert_eq!(cnf.clauses, vec![
///   vec![1, -5, 4],
///   vec![-1, 5, 3, 4],
///   vec![-3, -4],
/// ]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CNF {
    pub num_variables: usize,
    pub num_clauses: usize,
    pub clauses: Vec<Vec<i32>>,
}

impl CNF {
    pub fn from_dimacs_format_str(input: &str) -> Result<Self> {
        Self::from_dimacs_format(input.as_bytes())
    }

    pub fn from_dimacs_format(input: impl io::Read) -> Result<Self> {
        let input = io::BufReader::new(input);
        let mut lines = input.lines().filter_map(|line| {
            let line = line.ok()?;
            let line = line.trim();
            if line.is_empty() {
                // Emtpy line is ignored
                return None;
            }
            if line.starts_with(['c', 'C']) {
                // Comment
                return None;
            }
            Some(line.split(' ').map(str::to_string).collect::<Vec<String>>())
        });
        let header = lines.next().context("Missing header")?;
        if header.len() != 4 || header[0].to_lowercase() != "p" || header[1].to_lowercase() != "cnf"
        {
            bail!("Invalid header: {}", header.join(" "));
        }
        let num_variables = header[2].parse::<usize>()?;
        let num_clauses = header[3].parse::<usize>()?;

        let clauses = lines
            .map(|line| {
                if let Some(last) = line.last() {
                    if last != "0" {
                        bail!("Missing terminator 0: {}", line.join(" "));
                    }
                }
                line.iter()
                    .take(line.len() - 1)
                    .map(|s| Ok(s.parse()?))
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(CNF {
            num_variables,
            num_clauses,
            clauses,
        })
    }
}
