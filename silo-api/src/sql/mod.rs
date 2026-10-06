pub enum SQLDataType {
    Char,
    VarChar,
    Int,
    Decimal,
}

pub struct Column {
    pub name: &'static str,
    pub data_type: SQLDataType,
}

pub struct Constraint {
    pub name: &'static str,
    pub columns: Vec<Column>,
}

pub struct Index {
    pub name: &'static str,
    pub columns: Vec<Column>,
}

pub struct Table {
    pub name: &'static str,
    pub columns: Vec<Column>,
    pub indexes: Vec<Index>,
    pub constraints: Vec<Constraint>,
}

pub struct Schema {
    pub name: &'static str,
    pub tables: Vec<Table>,
}

pub struct View {
    pub name: &'static str,
    pub columns: Vec<Column>,
}
