pub struct BasicBlock {
    pub left: Option<Box<BasicBlock>>,
    pub right: Option<Box<BasicBlock>>,
}

pub struct Function {
    pub entry: Box<BasicBlock>,
}

impl Function {
    pub fn new(entry: Box<BasicBlock>) -> Function {
        Self { entry: entry }
    }
}

pub struct FunctionStore {
    pub functions: Vec<Function>,
}

impl FunctionStore {
    pub fn new() -> FunctionStore {
        Self {
            functions: Vec::new(),
        }
    }
}
