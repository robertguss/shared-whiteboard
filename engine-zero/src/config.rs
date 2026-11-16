use crate::Result;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct Config {
    pub machine: String,
    pub remote: String,
    pub sources: BTreeMap<String, PathBuf>,
    pub t3_database: PathBuf,
    pub record_folder: PathBuf,
    pub database: PathBuf,
    #[serde(default)]
    pub project_parents: Vec<String>,
    #[serde(default)]
    pub temporary_prefixes: Vec<String>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Config> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read config {}: {e}", path.display()))?;
        let config: Config = serde_json::from_str(&text)?;
        if config.machine.is_empty() || config.machine.contains('/') {
            return Err("config needs a machine name without '/'".into());
        }
        let repository = real(&Path::new(env!("CARGO_MANIFEST_DIR")).join(".."));
        for path in [&config.record_folder, &config.database] {
            if real(&std::path::absolute(path)?).starts_with(&repository) {
                return Err(format!(
                    "{} is inside the repository {}; private files must live outside it",
                    path.display(),
                    repository.display()
                )
                .into());
            }
        }
        Ok(config)
    }
}

fn real(path: &Path) -> PathBuf {
    match (path.canonicalize(), path.parent(), path.file_name()) {
        (Ok(resolved), _, _) => resolved,
        (Err(_), Some(parent), Some(name)) => real(parent).join(name),
        _ => path.to_path_buf(),
    }
}
