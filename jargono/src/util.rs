#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Minus,
    Return,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Bool, // i1
    Int,  // i32
}

#[derive(Debug, PartialEq)]
pub struct Argument {
    name: String,
    value_type: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Int(i32),
    Bool(bool),

    Ref(String),
    Assign(String, Box<Node>),

    // name (arguments) -> return_value { children }
    Function {
        name: String,
        arguments: Vec<Argument>,
        return_value: Type,
        children: Vec<Box<Node>>,
    },

    UnaryExpr {
        op: UnaryOp,
        child: Box<Node>,
    },
    BinaryExpr {
        op: BinaryOp,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}
