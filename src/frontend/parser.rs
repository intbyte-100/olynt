use std::{io, str::CharIndices};

use crate::{
    frontend::ast::{ASTGenerator, AST},
    frontend::chariter::CharIterator,
    util::logger::{self, Logger},
};

use super::{token::Token, util::is_special_symbol};

type Lines = io::Lines<io::BufReader<std::fs::File>>;

pub struct Parser<'a> {
    line_index: u32,
    lines: &'a mut Lines,
    logger: &'a mut Logger,
}

impl<'a> Parser<'a> {
    pub fn new(lines: &'a mut Lines, logger: &'a mut Logger) -> Self {
        Parser {
            line_index: 1,
            lines,
            logger,
        }
    }

    pub fn parse(&mut self) -> AST {
        let mut ast = AST::new();
        let mut ast_generator = ASTGenerator::new(self.logger, &mut ast);

        for i in &mut *self.lines {
            let line = i.unwrap();
            let info = (self.line_index, &line);

            {
                let mut parser = LineLexer::new(&line);

                while let Some(token) = parser.next() {
                    ast_generator.push(&info, Token::parse(self.line_index, token))
                }

                ast_generator.push(&info, Token::NewLine(self.line_index))
            }

            self.line_index += 1;
        }

        ast
    }
}

pub struct LineLexer<'a> {
    string_source: &'a String,
    token_start: usize,
    char_iterator: CharIterator<CharIndices<'a>>,
}

impl<'a> LineLexer<'a> {
    pub(crate) fn new(string: &'a String) -> Self {
        LineLexer {
            string_source: string,
            token_start: 0,
            char_iterator: CharIterator::new(string.char_indices()),
        }
    }

    #[inline]
    pub(crate) fn has_next(&mut self) -> bool {
        let has = self.char_iterator.next().is_some();
        self.char_iterator.go_back();
        has
    }

    #[inline]
    fn trim(&mut self) {
        while let Some(i) = self.char_iterator.next() {
            match i.1 {
                ' ' | '\n' | '\t' => self.token_start += 1,
                _ => {
                    self.char_iterator.go_back();
                    break;
                }
            }
        }
    }

    #[inline]
    fn is_space(i: char) -> bool {
        match i {
            ' ' | '\n' | '\t' => true,
            _ => false,
        }
    }

    #[inline]
    pub(crate) fn next(&mut self) -> Option<&str> {
        self.has_next().then(|| self.string_token())
    }

    pub(crate) fn string_token(&mut self) -> &str {
        self.trim();

        let fisrt_char = self.char_iterator.next().unwrap();

        let end = loop {
            if let Some((i, c)) = self.char_iterator.next() {
                if LineLexer::is_space(c) {
                    break i;
                }
                if is_special_symbol(c) {
                    if i - fisrt_char.0 > 1 {
                        self.char_iterator.go_back();
                        break i;
                    }
                }
            } else {
                break self.string_source.len();
            }
        };

        self.trim();
        &&self.string_source[fisrt_char.0..end]
    }
}



