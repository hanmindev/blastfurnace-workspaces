use crate::module::Path;
use std::fmt::Display;

/// A name. This is the unique identifier for a definition.
#[derive(Debug, Clone)]
pub struct Name {
    ident: String,
    path: Path,
    id: usize,
}

impl Name {
    /// Create a new name.
    pub fn new(ident: String, path: Path, id: usize) -> Name {
        Name { ident, path, id }
    }

    /// Get the identifier of the name.
    pub fn ident(&self) -> &str {
        self.ident.as_str()
    }

    /// Get the module path of the name
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the unique id of the name.
    pub fn id(&self) -> usize {
        self.id
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}::{}_{}", self.path, self.ident, self.id)
    }
}
