use crate::{frontend::util::is_special_symbol, util::mask::Mask};

pub(crate) enum Token {
    Word(u32, String),
    Number(u32, String),
    Operator(u32, char),
    NewLine(u32),
}

impl Token {
    pub(crate) fn parse(line_index: u32, string: &str) -> Token {
        let c = string.chars().nth(0).unwrap();
        if is_special_symbol(c) {
            Token::Operator(line_index, c)
        } else {
            let mut is_number = true;
            for i in string.chars() {
                if !(i >= '0' && i <= '9') {
                    is_number = false;
                    break;
                }
            }

            if is_number {
                Token::Number(line_index, string.to_string())
            } else {
                Token::Word(line_index, string.to_string())
            }
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            Token::Word(_, name) => format!("word '{}'", name),
            Token::Number(_, number) => format!("number '{}'", number),
            Token::Operator(_, op) => format!("operator '{}'", op),
            Token::NewLine(_) => "NewLine".to_string(),
        }
    }
}

pub(crate) enum MatcherFlag {
    NotEquals = 0,
    AnotherType,
}

pub(crate) struct MatcherMask {
    mask: Mask<i8>,
}

impl MatcherMask {
    fn new() -> Self {
        Self { mask: Mask::new() }
    }
    #[inline]
    fn enable(&mut self, flag: MatcherFlag) {
        self.mask.set(flag as i8, true)
    }

    #[inline]
    pub fn get_state(&self, flag: MatcherFlag) -> bool {
        self.mask.get(flag as i8)
    }

    pub fn is_ok(&self) -> bool {
        self.mask.as_int() == 0
    }
}

pub(crate) struct WordTokenMatcher {
    pub mask: MatcherMask,
    pub token: Option<String>,
    line: u32,
}

impl WordTokenMatcher {
    fn new() -> Self {
        Self {
            mask: MatcherMask::new(),
            token: None,
            line: 0,
        }
    }
    pub fn from(token: Token) -> Self {
        let mut matcher = Self::new();
        match token {
            Token::Word(line, string) => {
                matcher.token = Some(string);
                matcher.line = line;
            }
            _ => {
                matcher.mask.enable(MatcherFlag::AnotherType);
                matcher.token = Some(token.to_string())
            }
        }
        matcher
    }

    pub fn equals(mut self, val: &str) -> Self {
        if let Some(token) = self.token.as_ref() {
            (token != val).then(|| { self.mask.enable(MatcherFlag::NotEquals)});
        }
        self
    }
}
