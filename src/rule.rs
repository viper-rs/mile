#[derive(Debug)]
pub enum MatchResult<T> {
    None,
    Match(Option<T>),
    PartialMatch,
}

impl<T> MatchResult<T> {
    pub const fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }

    pub const fn is_match(&self) -> bool {
        match self {
            Self::Match(_) => true,
            _ => false,
        }
    }

    pub const fn is_partial_match(&self) -> bool {
        match self {
            Self::PartialMatch => true,
            _ => false,
        }
    }
}

/// Represents a rule for text matching.
#[derive(Debug)]
pub enum Rule<'a, T: core::fmt::Debug> {
    /// Matches if equal to the provided literal.
    Literal(&'a str),
    /// Matches if all characters are numeric.
    Numeric,
    /// Matches if all characters are alphabetic.
    Alphabetic,
    /// Matches if all characters are whitespace.
    Whitespace,
    /// Value extraction if matching the provided rule.
    Value(&'a Rule<'a, T>, fn(&'a str) -> T),
    /// Ignores if the provided rule matches.
    Ignore(&'a Rule<'a, T>),
    /// Matches if the ending matches provided literal.
    EndsWith(&'a str),
    /// Matches if the provided rule doesn't match.
    Not(&'a Rule<'a, T>),
    /// Matches if only the provided rule matches.
    Only(&'a Rule<'a, T>),
    /// Matches if both of the provided rules match.
    Both(&'a Rule<'a, T>, &'a Rule<'a, T>),
    /// Matches if either of the provided rules match.
    Either(&'a Rule<'a, T>, &'a Rule<'a, T>),
    /// Matches if all of the provided rules match.
    All(&'a [Rule<'a, T>], fn(&'a str) -> T),
    /// Matches if any of the provided rules match.
    Any(&'a [Rule<'a, T>]),
}

impl<'a, T: core::fmt::Debug> Rule<'a, T> {
    pub fn matches(&self, value: &'a str) -> MatchResult<T> {
        match self {
            Self::Literal(literal) => value
                .eq(*literal)
                .then_some(MatchResult::Match(None))
                .unwrap_or(
                    literal
                        .starts_with(value)
                        .then_some(MatchResult::PartialMatch)
                        .unwrap_or(MatchResult::None),
                ),
            Self::Numeric => value
                .chars()
                .all(|c| c.is_numeric())
                .then_some(MatchResult::Match(None))
                .unwrap_or(MatchResult::None),
            Self::Alphabetic => value
                .chars()
                .all(|c| c.is_alphabetic())
                .then_some(MatchResult::Match(None))
                .unwrap_or(MatchResult::None),
            Self::Whitespace => value
                .chars()
                .all(|c| c.is_whitespace())
                .then_some(MatchResult::Match(None))
                .unwrap_or(MatchResult::None),
            Self::Value(rule, out) => rule
                .matches(value)
                .is_match()
                .then_some(MatchResult::Match(Some(out(value))))
                .unwrap_or(
                    rule.matches(value)
                        .is_partial_match()
                        .then_some(MatchResult::PartialMatch)
                        .unwrap_or(MatchResult::None),
                ),
            Self::Ignore(rule) => rule.matches(value),
            Self::EndsWith(literal) => value
                .ends_with(literal)
                .then_some(MatchResult::Match(None))
                .unwrap_or(MatchResult::None),
            Self::Not(rule) => rule
                .matches(value)
                .is_none()
                .then_some(MatchResult::Match(None))
                .unwrap_or(MatchResult::None),
            Self::Only(rule) => rule.matches(value),
            Self::Both(a, b) => a
                .matches(value)
                .is_match()
                .then_some(
                    b.matches(value)
                        .is_match()
                        .then_some(MatchResult::Match(None))
                        .unwrap_or(MatchResult::None),
                )
                .unwrap_or(MatchResult::None),
            Self::Either(a, b) => a
                .matches(value)
                .is_match()
                .then_some(MatchResult::Match(None))
                .unwrap_or(
                    a.matches(value)
                        .is_partial_match()
                        .then_some(MatchResult::PartialMatch)
                        .unwrap_or(
                            b.matches(value)
                                .is_match()
                                .then_some(MatchResult::Match(None))
                                .unwrap_or(
                                    b.matches(value)
                                        .is_partial_match()
                                        .then_some(MatchResult::PartialMatch)
                                        .unwrap_or(MatchResult::None),
                                ),
                        ),
                ),
            Self::All(rules, out) => {
                for rule in *rules {
                    match rule.matches(value) {
                        MatchResult::None => return MatchResult::None,
                        MatchResult::PartialMatch => return MatchResult::PartialMatch,
                        _ => {}
                    }
                }

                MatchResult::Match(Some(out(value)))
            }
            Self::Any(rules) => {
                let mut matches = 0;

                for rule in *rules {
                    match rule.matches(value) {
                        MatchResult::None => {}
                        MatchResult::Match(token) => {
                            if matches == 0 {
                                return MatchResult::Match(token);
                            }
                        }
                        MatchResult::PartialMatch => matches += 1,
                    }
                }

                MatchResult::None
            }
        }
    }
}
