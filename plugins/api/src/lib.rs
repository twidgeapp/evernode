use anyhow::Result;
use sdk::{Connector, ConnectorTypes};
use sqlite::SQLiteConnector;
pub struct API {
    sql_connector: SQLiteConnector,
}

impl API {
    pub fn new() -> Self {
        let sqlite_connector = sqlite::SQLiteConnector::new();
        Self {
            sql_connector: sqlite_connector,
        }
    }

    pub async fn connect(&mut self, connection_url: &str, connection_type: &str) -> Result<()> {
        let conn_type = match connection_type {
            "Sqlite" => ConnectorTypes::SQLite,
            _ => todo!(),
        };

        match conn_type {
            sdk::ConnectorTypes::SQLite => self.sql_connector.connect(connection_url).await,
        };

        todo!()
    }
}
