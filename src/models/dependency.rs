// src/models/dependency.rs
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version_req: String, // e.g. "^1.40.0"
    pub kind: DepKind,
    // later: features, optional, source, etc.
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepKind {
    Normal,
    Development,
    Build,
}

impl Dependency {
    pub fn compact_line(&self) -> String {
        format!("{} {}", self.name, self.version_req)
    }
}
