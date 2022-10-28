use anyhow::Result;
use sdk::Connector;
use serde_json::{Map, Value};

pub struct SQLiteConnector {
    conn: Option<rusqlite::Connection>,
}

impl SQLiteConnector {
    pub fn new() -> Self {
        Self { conn: None }
    }
}

#[async_trait::async_trait]
impl Connector for SQLiteConnector {
    async fn connect(&mut self, connection_url: &str) -> Result<()> {
        let connection = rusqlite::Connection::open(connection_url)?;
        self.conn = Some(connection);

        Ok(())
    }

    async fn exec(&mut self, query: &str) -> Result<Vec<Map<String, Value>>> {
        let conn = self.conn.as_mut().unwrap();
        let mut stmt = conn.prepare(query)?;

        let rows = stmt.query_map([], |row| {
            let mut map = serde_json::Map::new();

            // Loop through all the columns in the row and add them to the map
            for i in 0..row.column_count() {
                let name = row.column_name(i).unwrap();

                // get the value of the column
                let value = row.get_ref(i)?;

                // change the value to a serde_json::Value
                let serde_value = match value {
                    rusqlite::types::ValueRef::Null => serde_json::Value::Null,
                    rusqlite::types::ValueRef::Integer(num) => {
                        serde_json::Value::Number(num.into())
                    }
                    rusqlite::types::ValueRef::Real(real) => {
                        serde_json::Value::String(real.to_string())
                    }
                    rusqlite::types::ValueRef::Text(text)
                    | rusqlite::types::ValueRef::Blob(text) => {
                        // convert text to string
                        let str = std::str::from_utf8(text)?;
                        serde_json::Value::String(str.to_string())
                    }
                };

                map.insert(name.to_string(), serde_value);
            }
            Ok(map)
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }

        Ok(result)
    }
}
