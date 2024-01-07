use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: Option<Thing>,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateForm {
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64,
}