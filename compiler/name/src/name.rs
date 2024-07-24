use crate::module_path::ModulePath;
use std::fmt::Display;

/// A name. This is the unique identifier for a definition.
pub struct Name {
    ident: String,
    path: ModulePath,
    id: usize,
}

impl Name {
    /// Create a new name.
    pub fn new(ident: String, path: ModulePath, id: usize) -> Name {
        Name { ident, path, id }
    }

    /// Get the identifier of the name.
    pub fn ident(&self) -> &str {
        self.ident.as_str()
    }

    /// Get the module path of the name
    pub fn path(&self) -> &ModulePath {
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
