#[derive(Debug, Clone)]
pub struct DataSourceNode {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pw: String,
    pub db_name: String,
}

impl DataSourceNode {
    pub fn new(name: &str, host: &str, port: &u16, user: &str, pw: &str, db_name: &str) -> Self {
        return Self {
            name: name.to_string(),
            host: host.to_string(),
            port: port.clone(),
            user: user.to_string(),
            pw: pw.to_string(),
            db_name: db_name.to_string(),
        };
    }
}
