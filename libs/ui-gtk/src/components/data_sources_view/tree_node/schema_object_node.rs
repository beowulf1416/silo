#[derive(Debug, Clone)]
pub struct SchemaObjectNode {
    pub name: String,
}

impl SchemaObjectNode {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
        };
    }
}
