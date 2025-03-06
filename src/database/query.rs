#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Identifier(pub(crate) String);

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Identifier(value)
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct QueryPlan {
    pub(crate) projection: Vec<Identifier>,
    pub(crate) table: Identifier,
}

impl QueryPlan {
    pub fn get_table_name(&self) -> String {
        self.table.0.clone()
    }
}

pub struct QueryPlanner {

}

impl QueryPlanner {
    pub fn new() -> Self {
        QueryPlanner {}
    }

    pub fn plan() {

    }
}

































