use crate::{BNumber, BnumGroup};

enum MonadicVerb {
    Negate,
    Abs,
}

enum DyadicVerb {
    Plus,
    Minus,
}

enum AstNode {
    Float(f32),
    BNumber(BNumber),
    BNumGroup(BnumGroup),
    MonadicOp {
        verb: MonadicVerb,
        expr: Box<AstNode>,
    },
    DyadicOp {
        verb: DyadicVerb,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    TriadicOp {
        verb: DyadicVerb,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
        arg: Box<AstNode>,
    },
    Type(String),
    Ident(String),
    Str(String),
}
