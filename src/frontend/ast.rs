use std::collections::LinkedList;

use crate::{
    error_log,
    frontend::token::Token,
    frontend::token::{self, MatcherFlag, WordTokenMatcher},
    util::logger::Logger,
};

use super::sentence::ExecutionBlock;

pub struct Function {
    blocks: Vec<ExecutionBlock>,
    name: String,
}
pub struct AST {
    compilation_failed: bool,
    submodule: Option<String>,
    functions: Vec<Function>,
}

impl AST {
    pub fn new() -> Self {
        AST {
            submodule: None,
            functions: Vec::new(),
            compilation_failed: false,
        }
    }
}

pub type LineInfo<'a> = (u32, &'a String);
pub(crate) struct ASTGenerator<'a, 'b> {
    token_buffer: LinkedList<Token>,
    logger: &'a mut Logger,
    line_info: Option<&'a (u32, &'b String)>,
    ast: &'a mut AST,
    func_declared: bool,
}

impl<'a, 'b> ASTGenerator<'a, 'b> {
    pub fn new(logger: &'a mut Logger, ast: &'a mut AST) -> Self {
        ASTGenerator {
            token_buffer: LinkedList::new(),
            logger: logger,
            line_info: None,
            ast: ast,
            func_declared: false,
        }
    }
    pub fn push(&mut self, line: &LineInfo, token: Token) {
        match token {
            Token::NewLine(_) => self.finish_sentence(line),
            _ => self.token_buffer.push_back(token),
        };
    }

    fn define_func(&mut self, line: &LineInfo) {}

    fn parse_func(&mut self, line: &LineInfo) {}

    fn define_submodule(&mut self, line: &LineInfo) {
        let token = self.token_buffer.pop_front();

        let token = match token {
            Some(x) => x,
            None => return,
        };

        let state = WordTokenMatcher::from(token).equals("submodule");
        
        if !state.mask.is_ok() {
            Some(error_log!(
                self.logger,
                line.0,
                0,
                "Syntax error: expected submodule definition, but '{}' provided",
                state.token.unwrap()
            ));

            return
        } 

        
    }

    pub fn finish_sentence(&mut self, line: &LineInfo) {
        if self.ast.submodule.is_none() {
            self.define_submodule(line);
        } else if self.func_declared {
            self.parse_func(line)
        } else {
            self.define_func(line)
        }
    }

    pub fn set_line_info(&mut self, line_info: Option<&'a (u32, &'b String)>) {
        self.line_info = line_info;
    }
}
