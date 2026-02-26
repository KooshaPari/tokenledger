use pheno_core::{ConfigStore, FlagStore};
use pheno_db::Database;
use std::path::Path;

/// Initialize the phenotype-config database for this repo.
/// Creates `.phenotype/config.db` under `repo_root` if it doesn't exist.
pub fn init(repo_root: &Path) -> pheno_core::Result<Database> {
    let db_path = repo_root.join(".phenotype").join("config.db");
    Database::open(&db_path)
}
