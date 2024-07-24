use std::fmt::Display;

/// An instruction.
#[derive(Debug, Clone)]
pub struct Instruction {
    pub kind: InstructionKind,
}

/// The various kinds of instructions the compiler recognizes
#[derive(Debug, Clone)]
pub enum InstructionKind {
    // A function call. Call(function, foo, { ... }) would be evaluated to the command
    // function bar { ... } assuming the name foo gets resolved to bar
    Call(String, String, String),
    // A standard Minecraft command. Do not prefix with /.
    Command(String),
    // A chain of instructions for commands like execute or return which run a command after
    // another command. e.g. Chain("execute as @a run", "say hi")
    Chain(Box<Instruction>, Box<Instruction>),
    // A block of instructions
    Block(Block),
}

/// A block containing instructions
#[derive(Debug, Clone)]
pub struct Block {
    pub instructions: Vec<Instruction>,
}

/// A function containing a name and a block
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub block: Block,
}

/// A hmasm file containing functions
///
/// This is the top-level structure of a hmasm file.
///
/// It contains a path to the file and a list of functions.
#[derive(Debug, Clone)]
pub struct File {
    pub path: blastf_name::module::Path,
    pub functions: Vec<Function>,
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "File: {}\n", self.path)?;
        for function in &self.functions {
            write!(f, "{}\n", function)?;
        }
        Ok(())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.block)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{\n")?;
        for instruction in &self.instructions {
            write!(f, "{}\n", instruction)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            InstructionKind::Call(first, func, last) => write!(f, "{} {} {}", first, func, last),
            InstructionKind::Command(cmd) => write!(f, "{}", cmd),
            InstructionKind::Chain(first, second) => write!(f, "{} {}", first, second),
            InstructionKind::Block(block) => write!(f, "{}", block),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let mut path = blastf_name::module::Path::new();
        path.push("root".to_string());
        path.push("foo".to_string());

        let block = Block {
            instructions: vec![Instruction {
                kind: InstructionKind::Command("say hi".to_string()),
            }],
        };
        let function = Function {
            name: "bar".to_string(),
            block,
        };
        let file = File {
            path,
            functions: vec![function],
        };

        assert_eq!(format!("{}", file), "File: root::foo\nbar {\nsay hi\n}\n");
    }
}
