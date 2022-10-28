use anyhow::Result;
use sdk::Connector;

pub struct SQLiteConnector {}

impl SQLiteConnector {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Connector for SQLiteConnector {
    async fn connect(&mut self, connection_url: &str) -> Result<()> {
        let connection = rusqlite::Connection::open(connection_url)?;

        let mut stmt = connection.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut names: Vec<String> = Vec::new();
        for name_result in rows {
            names.push(name_result?);
        }

        println!("Tables: {:?}", names);

        todo!()
    }
}
