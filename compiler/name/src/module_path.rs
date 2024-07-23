use std::fmt::Display;

/// A path. This is the location of a module.
///
/// This is a series of segments separated by `::`.
///
/// E.g., `root::foo::bar`.
pub struct ModulePath {
    segments: Vec<ModulePathSegment>,
}

/// A segment of a path.
///
/// E.g., `foo` in `root::foo::bar`.
struct ModulePathSegment {
    ident: String,
}

impl ModulePath {
    /// Create a new, empty path.
    pub fn new() -> ModulePath {
        ModulePath {
            segments: Vec::new(),
        }
    }

    /// Push an ident segment onto the path.
    pub fn push(&mut self, ident: String) {
        self.segments.push(ModulePathSegment { ident });
    }

    /// Get the root segment of the path, i.e. the first segment.
    /// E.g., `root` in `root::foo::bar`.
    ///
    /// # Panics
    ///
    /// Panics if the name is empty.
    pub fn get_root(&self) -> &str {
        self.segments.first().unwrap().ident.as_str()
    }

    /// Get the last segment of the path.
    /// E.g. `Bar` in `root::foo::Bar`.
    ///
    /// # Panics
    ///
    /// Panics if the path is empty.
    pub fn get_last(&self) -> &str {
        self.segments.last().unwrap().ident.as_str()
    }

    /// Pop the root segment off the path.
    /// E.g., `root` in `root::foo::bar`.
    ///
    /// # Panics
    ///
    /// Panics if the path is empty.
    pub fn pop_root(&mut self) {
        self.segments.remove(0);
    }

    /// Extend this path with another path.
    pub fn extend(&mut self, other: ModulePath) {
        self.segments.extend(other.segments);
    }
}

impl Display for ModulePath {
    /// The name is displayed as a series of segments separated by ::
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, segment) in self.segments.iter().enumerate() {
            if i > 0 {
                write!(f, "::")?;
            }
            write!(f, "{}", segment.ident)?;
        }
        Ok(())
    }
}
