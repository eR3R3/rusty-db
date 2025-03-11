use std::collections::HashMap;
use crate::database::query::QueryPlan;
use crate::database::schema::Row;
use crate::database::storage_engine::StorageEngine;
use crate::error::{Error, ExecutionError};

pub struct ExecutionEngine {
    storage_engine: StorageEngine,
}

impl ExecutionEngine {
    pub fn new(storage_engine: StorageEngine) -> Self {
        ExecutionEngine { storage_engine }
    }

    pub fn execute(&self, query_plan: QueryPlan) -> Result<Vec<Row>, Error> {
        let mut table = self
            .storage_engine.tables.get(&query_plan.get_table_name())
            .ok_or(ExecutionError::ReadError("cannot read table name from query plan".to_string()))?;
        let mut result = Vec::new();
        for row in table.rows.values() {
            let mut row_data = HashMap::new();
            for column in &query_plan.projection {
                row_data.insert(column.0.clone(),
                row.data.get(&column.0).cloned()
                    .ok_or(ExecutionError::ReadError("cannot read column name from query plan".to_string()))?);
            }
            result.push(Row { data: row_data })
        };
        Ok(result)
    }
}































