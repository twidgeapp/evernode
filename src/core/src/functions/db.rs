use anyhow::Result;

use crate::prisma::{self, PrismaClient};

pub async fn new_client() -> Result<PrismaClient> {
    let evernode_dir = dirs::home_dir().unwrap().join(".evernode");

    std::fs::create_dir_all(evernode_dir.clone())?;

    let db_path = evernode_dir.join("library.db");

    log::info!("Database path: {}", db_path.display());

    if !db_path.exists() {
        std::fs::File::create(db_path.clone())?;
    }

    let db_url = &format!("file:{}", db_path.display());
    let database = prisma::new_client_with_url(&format!("file:{}", db_path.display())).await?;

    #[cfg(debug_assertions)]
    database._db_push().await?;

    #[cfg(not(debug_assertions))]
    database._migrate_deploy().await?;

    Ok(database)
}
