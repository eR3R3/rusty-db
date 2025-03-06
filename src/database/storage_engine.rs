use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::database::schema::{Row, Table};
use bincode;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct StorageEngine {
    pub tables: HashMap<String, Table>
}

impl StorageEngine {
    pub fn new() -> Self {
        StorageEngine {
            tables: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, name: &str, columns: Vec<String>) {
        self.tables.insert(name.to_string(), Table { columns, rows: HashMap::new() });
    }

    pub fn insert_row(&mut self, name: &str, row: Row) -> Result<(), String> {
        // It is better to use match here
        // if let Some(table) = self.tables.get_mut(name) {
        //     let row_id = table.rows.len();
        //     table.rows.insert(row_id, row);
        // };
        match self.tables.get_mut(name) {
            Some(table) => {
                let row_id = table.rows.len();
                table.rows.insert(row_id, row);
                Ok(())
            }
            _ => { Err(format!("no tables found with name {name}")) }
        }
    }

    pub fn serialize(&self, buffer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buffer.clear();
        buffer.extend(bincode::serialize(self).unwrap());
        Ok(())
    }

    pub fn deserialize(&self, buffer: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(buffer)
    }
}



















