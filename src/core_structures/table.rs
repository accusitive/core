use std::collections::HashMap;
use crate::core_structures::column::ColumnType;
use crate::core_structures::row::Row;

#[derive(Debug, Clone)]
pub struct Table {
    pub columns: HashMap<String, ColumnType>,
    pub rows: HashMap<String, Row>
}