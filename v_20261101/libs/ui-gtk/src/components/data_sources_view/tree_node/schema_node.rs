#[derive(Debug, Clone)]
pub struct SchemaNode {
    pub name: String,
}

impl SchemaNode {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
        };
    }
}
