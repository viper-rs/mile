#![forbid(unsafe_code)]

pub mod rule;

pub use rule::*;

#[derive(Debug)]
pub enum Error<'a> {
    None,
    Eof,
    UnknownToken(&'a str),
}

pub type Result<'a, T> = core::result::Result<T, Error<'a>>;

pub struct Lexer<'a, T: core::fmt::Debug> {
    data: &'a str,
    buffer: &'a str,
    rule: Rule<'a, T>,
    index: (usize, usize),
}

impl<'a, T: core::fmt::Debug> Lexer<'a, T> {
    pub const fn new(rule: Rule<'a, T>) -> Self {
        Self {
            data: "",
            buffer: "",
            rule,
            index: (0, 0),
        }
    }

    pub const fn with_buffer(rule: Rule<'a, T>, buffer: &'a str) -> Self {
        Self {
            data: "",
            buffer,
            rule,
            index: (0, 0),
        }
    }

    pub fn reset(&mut self, buffer: &'a str) {
        self.data = "";
        self.buffer = buffer;
        self.index = (0, 0);
    }

    pub fn step(&mut self) -> Result<Option<T>> {
        self.index.1 += 1;

        if self.index.1 >= self.buffer.len() {
            return Err(Error::Eof);
        }

        self.data = &self.buffer[self.index.0..self.index.1];

        println!("Data: `{}`", self.data);

        match self.rule.matches(self.data) {
            MatchResult::None | MatchResult::PartialMatch => Ok(None),
            MatchResult::Match(token) => {
                self.index.0 = self.index.1;
                Ok(token)
            }
        }
    }
}

impl<'a, T: core::fmt::Debug> Iterator for Lexer<'a, T> {
    type Item = Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.step() {
            Ok(token) => Some(token),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CODE: &'static str = r#"
function add(a, b)
    return a + b
end
"#;

    #[derive(Debug)]
    enum Token<'a> {
        And,
        Break,
        Do,
        Else,
        ElseIf,
        End,
        False,
        For,
        Function,
        If,
        In,
        Local,
        Nil,
        Not,
        Or,
        Repeat,
        Return,
        Then,
        True,
        Until,
        While,
        Identifier(&'a str),
    }

    #[test]
    fn it_works() {
        let mut lexer = Lexer::<Token>::with_buffer(
            Rule::Any(&[
                Rule::Ignore(&Rule::Whitespace),
                Rule::Value(&Rule::Literal("and"), |_| Token::And),
                Rule::Value(&Rule::Literal("break"), |_| Token::Break),
                Rule::Value(&Rule::Literal("do"), |_| Token::Do),
                Rule::Value(&Rule::Literal("else"), |_| Token::Else),
                Rule::Value(&Rule::Literal("elseif"), |_| Token::ElseIf),
                Rule::Value(&Rule::Literal("end"), |_| Token::End),
                Rule::Value(&Rule::Literal("false"), |_| Token::False),
                Rule::Value(&Rule::Literal("for"), |_| Token::For),
                Rule::Value(
                    &Rule::Either(&Rule::Literal("function"), &Rule::Literal("func")),
                    |_| Token::Function,
                ),
                Rule::Value(&Rule::Literal("if"), |_| Token::If),
                Rule::Value(&Rule::Literal("in"), |_| Token::In),
                Rule::Value(&Rule::Literal("local"), |_| Token::Local),
                Rule::Value(&Rule::Literal("nil"), |_| Token::Nil),
                Rule::Value(&Rule::Literal("not"), |_| Token::Not),
                Rule::Value(&Rule::Literal("or"), |_| Token::Or),
                Rule::Value(&Rule::Literal("repeat"), |_| Token::Repeat),
                Rule::Value(&Rule::Literal("return"), |_| Token::Return),
                Rule::Value(&Rule::Literal("then"), |_| Token::Then),
                Rule::Value(&Rule::Literal("true"), |_| Token::True),
                Rule::Value(&Rule::Literal("until"), |_| Token::Until),
                Rule::Value(&Rule::Literal("while"), |_| Token::While),
                Rule::Value(&Rule::Alphabetic, |v| Token::Identifier(v)),
            ]),
            TEST_CODE,
        );

        for token in lexer {
            if let Some(token) = token {
                println!("Token: {token:?}");
            }
        }
    }
}
