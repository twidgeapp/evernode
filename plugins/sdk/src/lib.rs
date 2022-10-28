use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Map, Value};

pub enum ConnectorTypes {
    SQLite,
}

#[async_trait]
pub trait Connector {
    async fn connect(&mut self, connection_url: &str) -> Result<()>;

    async fn exec(&mut self, query: &str) -> Result<Vec<Map<String, Value>>>;
}
