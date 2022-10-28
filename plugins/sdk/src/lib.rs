use anyhow::Result;
use async_trait::async_trait;

pub enum ConnectorTypes {
    SQLite,
}

#[async_trait]
pub trait Connector {
    async fn connect(&mut self, connection_url: &str) -> Result<()>;
}
