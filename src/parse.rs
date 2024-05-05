use anyhow::{bail, Context, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CNF {
    pub num_variables: usize,
    pub num_clauses: usize,
    pub clauses: Vec<Vec<i32>>,
}

impl CNF {
    pub fn from_dimacs_format(input: &str) -> Result<Self> {
        let mut lines = input.lines().filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                // Emtpy line is ignored
                return None;
            }
            if line.starts_with(['c', 'C']) {
                // Comment
                return None;
            }
            Some(line.split(' ').collect::<Vec<&str>>())
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
                line.iter()
                    .map(|&s| Ok(s.parse()?))
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
