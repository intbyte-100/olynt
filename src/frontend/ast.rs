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


pub(crate) struct ASTGenerator<'a> {
    token_buffer: LinkedList<Token>,
    logger: &'a mut Logger,
    ast: &'a mut AST,
    func_declared: bool,
}

impl<'a, 'b> ASTGenerator<'a> {
    pub fn new(logger: &'a mut Logger, ast: &'a mut AST) -> Self {
        ASTGenerator {
            token_buffer: LinkedList::new(),
            logger: logger,
            ast: ast,
            func_declared: false,
        }
    }
    pub fn push(&mut self, line: &u32, token: Token) {
        match token {
            Token::NewLine(_) => self.finish_sentence(line),
            _ => self.token_buffer.push_back(token),
        };
    }

    fn define_func(&mut self, line: &u32) {}

    fn parse_expresion(&mut self, line: &u32) {}

    fn define_submodule(&mut self, line: &u32) {
        if self.ast.submodule.is_some() {
            self.ast.compilation_failed = true;
            error_log!(
                self.logger,
                *line,
                0,
                "Submodule error: module already has defined"
            )
        }

        if let Some(token) = self.token_buffer.pop_front() {
            let state = WordTokenMatcher::from(token).equals("submodule");

            if !state.mask.is_ok() {
                self.ast.compilation_failed = true;
                error_log!(
                    self.logger,
                    *line,
                    0,
                    "Submodule error: expected submodule definition, but '{}' provided",
                    state.token.unwrap()
                );
            }
        } else {
            return;
        }

        if let Some(token) = self.token_buffer.pop_front() {
            if let Token::Word(_, name) = token {
                self.ast.submodule = Some(name);
                return;
            }
            self.ast.compilation_failed = true;
            error_log!(
                self.logger,
                *line,
                0,
                "Submodule error: invalid submodule name"
            );
        }
    }

    pub fn finish_sentence(&mut self, line: &u32) {
        if self.ast.submodule.is_none() {
            self.define_submodule(line);
        } else if self.func_declared {
            self.parse_expresion(line)
        } else {
            self.define_func(line)
        }
    }
}
