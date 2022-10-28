pub mod arguments;
pub mod commands;
pub mod shared;

use std::sync::Arc;

use anyhow::Result;

async fn try_main() -> Result<()> {
    let arguments = arguments::Arguments::parse()?;
    let client = node_core::functions::db::new_client().await?;

    let shared = shared::Shared {
        argument: arguments,
        prisma: Arc::new(client),
    };

    let commands = commands::Commands::new(shared);
    commands.exec().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "debug");

    pretty_env_logger::init();

    try_main().await.unwrap();
}
