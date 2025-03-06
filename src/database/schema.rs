use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct Table {
    pub(crate) columns: Vec<String>,
    pub(crate) rows: HashMap<usize, Row>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct Row {
    pub(crate) data: HashMap<String, String>,
}

