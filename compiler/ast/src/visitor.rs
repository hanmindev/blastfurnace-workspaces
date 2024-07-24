use crate::ast::*;
/// The current file sets up the infrastructure for the visitor pattern.
///
/// The visitor will recursively visit every node in the AST. A developer may implement a visitor to perform operations on the AST.

/// This section defines the ASTNodeEnum, which is an enum that contains every node in the AST.
macro_rules! visit_enum {
    ($($ast_type: ident), * ) => {
pub enum ASTNodeEnum<'a> {
        $(
            $ast_type(&'a mut $ast_type),
        ) *
    }
    };
}
visit_enum![
    Ident,
    Path,
    PathSegment,
    Definition,
    Ty,
    Struct,
    StructField,
    Fn,
    FnSig,
    FnHeader,
    FnDecl,
    Param,
    Expr,
    Constant,
    LocalBind,
    Statement,
    Block
];

/// The result of the visitor.
///
/// The boolean indicates whether the visitor should visit the children of the node.
/// `K` indicates an optional value that the visitor can return to the parent.
/// `V` is the error type.
pub type GenericVisitApplyResult<K, V> = Result<(bool, Option<K>), V>;

/// The visitor trait. This trait is implemented by the developer to perform operations on the AST.
/// It must be implemented for every node in the AST that the developer wants to perform operations on.
pub trait Visitor<K, V> {
    /// The default implementation of apply will return true and None.
    ///
    /// See the documentation for `GenericVisitApplyResult` for more information on what the return type does.
    fn apply(&mut self, _ast_node: &mut ASTNodeEnum) -> GenericVisitApplyResult<K, V> {
        return Ok((true, None));
    }
}

/// The Visitable trait. This trait is implemented for every node in the AST. It allows the visitor to
/// automatically visit every node in the AST.
pub trait Visitable<T: Visitor<K, V>, K, V> {
    /// Needs to be implemented for every node in the AST. Allows the visitor to automatically visit every node in the AST.
    ///
    /// When visiting a node, the visitor will call the `apply` method on the visitor. If the visitor returns true,
    /// the visitor will visit the children of the node.
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V>;
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for Ident {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        Ok(visitor.apply(&mut ASTNodeEnum::Ident(self))?.1)
    }
}

// This is a good example of how to implement the Visitable trait for a node in the AST that has children.
impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for Path {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        // first apply the visitor to the node
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::Path(self))?;

        // then if the boolean is true, visit the children of the node
        if visit_result {
            for segment in self.segments.iter_mut() {
                segment.visit(visitor)?;
            }
        }
        Ok(res)
    }
}

// This is a good example of how to implement the Visitable trait for a node in the AST that has no children.
impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for PathSegment {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        // just apply the visitor to the node, and return the result. Ignore whether the boolean is true or false since
        // there are no children to visit.
        Ok(visitor.apply(&mut ASTNodeEnum::PathSegment(self))?.1)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for Definition {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::Definition(self))?;
        if visit_result {
            match self.kind {
                DefinitionKind::Struct(ref mut s) => s.visit(visitor)?,
                DefinitionKind::Fn(ref mut f) => f.visit(visitor)?,
            };
        }
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for Ty {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::Ty(self))?;
        if visit_result {
            match &mut self.kind {
                TyKind::Path(ref mut p) => {
                    p.visit(visitor)?;
                }
                _ => {}
            };
        };
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for Struct {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::Struct(self))?;
        if visit_result {
            for field in self.fields.iter_mut() {
                field.visit(visitor)?;
            }
        }
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for StructField {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::StructField(self))?;
        if visit_result {
            self.ident.visit(visitor)?;
            self.ty.visit(visitor)?;
        }
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for Fn {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::Fn(self))?;
        if visit_result {
            self.sig.visit(visitor)?;
            if let Some(ref mut body) = self.body {
                body.visit(visitor)?;
            }
        }
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for FnSig {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::FnSig(self))?;
        if visit_result {
            self.header.visit(visitor)?;
            self.decl.visit(visitor)?;
        }
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for FnHeader {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        Ok(visitor.apply(&mut ASTNodeEnum::FnHeader(self))?.1)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for FnDecl {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::FnDecl(self))?;
        if visit_result {
            for param in self.inputs.iter_mut() {
                param.visit(visitor)?;
            }
        }
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for Param {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::Param(self))?;
        if visit_result {
            self.ty.visit(visitor)?;
        }
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for Expr {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::Expr(self))?;
        if visit_result {
            match self.kind {
                ExprKind::Constant(ref mut c) => c.visit(visitor)?,
                ExprKind::Variable(ref mut p) => p.visit(visitor)?,
                ExprKind::Block(ref mut b) => b.visit(visitor)?,
            };
        }
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for Constant {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        Ok(visitor.apply(&mut ASTNodeEnum::Constant(self))?.1)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for LocalBind {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::LocalBind(self))?;
        if visit_result {
            match self.kind {
                LocalBindKind::Decl => {}
                LocalBindKind::Init(ref mut e) => {
                    e.visit(visitor)?;
                }
            }
        };
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for Statement {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::Statement(self))?;
        if visit_result {
            match self.kind {
                StatementKind::Let(ref mut l) => l.visit(visitor)?,
                StatementKind::Def(ref mut d) => d.visit(visitor)?,
                StatementKind::Expr(ref mut e) => e.visit(visitor)?,
            };
        };
        Ok(res)
    }
}

impl<T: Visitor<K, V>, K, V> Visitable<T, K, V> for Block {
    fn visit(&mut self, visitor: &mut T) -> Result<Option<K>, V> {
        let (visit_result, res) = visitor.apply(&mut ASTNodeEnum::Block(self))?;
        if visit_result {
            for statement in self.stmts.iter_mut() {
                statement.visit(visitor)?;
            }
        }
        Ok(res)
    }
}
