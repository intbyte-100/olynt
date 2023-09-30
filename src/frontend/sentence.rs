pub(crate) enum SentenceType {
    Call(Box<CallSentecne>),
    Iter(Box<IterSentence>),
    Range(Box<RangeSentence>),
    Variable(Box<VariableDeclareSentence>),
    Const(Box<VariableDeclareSentence>),
    Final(Box<VariableDeclareSentence>),
    ExecutionBlock(Box<ExecutionBlock>),
}

pub(crate) enum ExpressionElement {
    Operator(char),
    Operand(i32),
}
pub(crate) struct ExpressionSentence {
    expr: Vec<ExpressionElement>,
    exprtype: i32,
}
pub(crate) struct ExecutionBlock {
    begin: i32,
    end: i32,
    next: i32,
}
pub(crate) struct VariableDeclareSentence {
    vartype: i32,
}
pub(crate) struct CallSentecne {
    func: i32,
    args: Vec<i32>,
}

pub(crate) struct IterSentence {
    range: i32,
    execution: i32,
}

pub(crate) struct RangeSentence {
    variable: i32,
    from: i32,
    to: i32,
    condition: i32,
    step: i32,
}
