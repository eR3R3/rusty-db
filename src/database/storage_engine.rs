use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::database::schema::{Row, Table};
use bincode;
use std::io::{ErrorKind, Read};
use crate::error::Error;
use crate::error::StorageEngineError::IOError;

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

    pub fn deserialize(buffer: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(buffer)
    }
}

pub struct FileSystem {
    pub storage_engine: StorageEngine,
    file_path: PathBuf,
}

impl FileSystem {
    fn new(file_path: &str) -> Result<Self, Error> {
        let file_path = PathBuf::from(file_path);
        let mut storage_engine = StorageEngine::new();
        if &file_path.exists() {
            if let Ok(tables) = FileSystem::read_from_file(file_path
                .to_str()
                .ok_or(Error::IO(std::io::Error::new(ErrorKind::InvalidData, "invalid data received from file")))?) {
                storage_engine = tables;
            }
        }
        Ok(
            FileSystem {
                storage_engine,
                file_path,
            }
        )
    }

    fn read_from_file(file_path: &str) -> Result<StorageEngine, Error> {
        let mut file = File::open(file_path)?;
        let mut buffer: Vec<u8> = Vec::new();
        let file_size = file.read(&mut buffer)?;
        let storage_engine = StorageEngine::deserialize(&buffer)?;
        Ok(storage_engine)
    }


}

























