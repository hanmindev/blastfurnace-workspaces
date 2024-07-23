use crate::ptr::P;
use std::fmt::Debug;

/// An identifier
type Ident = String;

/// A "name".
///
/// E.g., `root::foo::Bar`.
#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    /// The segments which are separated by ::
    pub segments: Vec<PathSegment>,
}

/// A segment of a path
///
/// E.g., `foo` in `root::foo::Bar`
#[derive(Debug, PartialEq, Clone)]
struct PathSegment {
    ident: Ident,
}

/// A definition (e.g., `struct`, `fn`)
#[derive(Debug, PartialEq, Clone)]
struct Definition<K = DefinitionKind> {
    ident: Ident,
    kind: K,
}

/// The various kinds of definitions the compiler recognizes
#[derive(Debug, PartialEq, Clone)]
enum DefinitionKind {
    /// A struct definition (`struct`).
    /// E.g. `struct Foo { ... }`
    Struct(Struct),
    /// A function declaration (`fn`).
    /// E.g. `fn foo(bar: baz) -> qux { ... }`
    Fn(Box<Fn>),
}

/// A type
#[derive(Debug, PartialEq, Clone)]
pub struct Ty {
    pub kind: TyKind,
}

/// The various kinds of types the compiler recognizes
#[derive(Debug, PartialEq, Clone)]
pub enum TyKind {
    Void,
    Int,
    Float,
    Bool,
    String,
    Path(Path),
}

/// A struct definition
#[derive(Debug, PartialEq, Clone)]
struct Struct {
    field: Vec<StructField>,
}
/// A struct field
///
/// E.g., `foo: bar` in `struct Foo { foo: bar }`
#[derive(Debug, PartialEq, Clone)]
struct StructField {
    ident: Ident,
    ty: P<Ty>,
}

/// A function definition
#[derive(Debug, PartialEq, Clone)]
struct Fn {
    sig: FnSig,
    body: Option<P<Block>>,
}

/// Represents a function's signature
#[derive(Debug, PartialEq, Clone)]
struct FnSig {
    header: FnHeader,
    decl: P<FnDecl>,
}

/// A function header
///
/// All the information that is not the visibility and the name of the function
/// (e.g., `const rec fn`)
#[derive(Debug, PartialEq, Clone)]
struct FnHeader {
    rec: bool,
    constness: bool,
}

/// Signature of the function declaration
/// E.g. fn foo(bar: baz) -> qux
#[derive(Debug, PartialEq, Clone)]
struct FnDecl {
    inputs: Vec<Param>,
    output: Ty,
}

/// A function parameter
///
/// E.g. `bar: baz` in `fn foo(bar: baz) -> qux`
#[derive(Debug, PartialEq, Clone)]
struct Param {
    ty: P<Ty>,
}

/// An expression
#[derive(Debug, PartialEq, Clone)]
struct Expr {
    kind: ExprKind,
}

/// The various kinds of expressions the compiler recognizes
#[derive(Debug, PartialEq, Clone)]
enum ExprKind {
    Variable(Path),
    Constant(Constant),
    Block(Block),
}

/// A constant
#[derive(Debug, PartialEq, Clone)]
struct Constant {
    kind: ConstantKind,
}

/// The various kinds of constants the compiler recognizes
/// E.g., `5`, `5.0`, `true`, `"foo"`
#[derive(Debug, PartialEq, Clone)]
enum ConstantKind {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

/// A local variable binding
#[derive(Debug, PartialEq, Clone)]
struct LocalBind {
    ident: Ident,
    ty: Option<Ty>,
    kind: LocalBindKind,
}

#[derive(Debug, PartialEq, Clone)]
enum LocalBindKind {
    /// Local variable binding
    /// E.g., `let x;`
    Decl,
    /// Local variable binding with initialization
    /// E.g., `let x = 5;`
    Init(Expr),
}

/// A statement. E.g., `return;`
#[derive(Debug, PartialEq, Clone)]
struct Statement {
    kind: StatementKind,
}

/// The various kinds of statements the compiler recognizes
#[derive(Debug, PartialEq, Clone)]
enum StatementKind {
    /// A local let binding
    Let(P<LocalBind>),
    /// A definition
    Def(P<Definition>),
    /// An expression
    Expr(P<Expr>),
}

/// A block ({ ... })
///
/// E.g., fn foo() { ... }
#[derive(Debug, PartialEq, Clone)]
struct Block {
    stmts: Vec<Statement>,
}
