#[derive(Debug, Clone)]
pub struct Silo {
    workspace_path: String,
}

impl Silo {
    pub fn new() -> Self {
        Self {
            workspace_path: String::from(""),
        }
    }

    pub fn set_workspace(&mut self, path: String) {
        self.workspace_path = path;
    }
}
