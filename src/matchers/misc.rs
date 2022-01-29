use std::{fmt::Debug};
use super::{Matcher, MatchOutput};

pub struct AnythingMatcher;

pub fn anything() -> AnythingMatcher {
    AnythingMatcher
}

impl <T> Matcher<T> for AnythingMatcher {
    fn matches(&self, _: T) -> MatchOutput {
        MatchOutput::Ok("_".to_string())
    }
}

impl Debug for AnythingMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("anything()").finish()
    }
}

pub struct NotMatcher<Inner>(Inner);

pub fn not<Inner>(matcher: Inner) -> NotMatcher<Inner> {
    NotMatcher(matcher)
}

impl <Inner, T> Matcher<T> for NotMatcher<Inner>
where
    Inner: Matcher<T>
{
    fn matches(&self, value: T) -> MatchOutput {
        match self.0.matches(value) {
            MatchOutput::Ok(ok) => {
                MatchOutput::Err(format!("Expected not {}", ok))
            },
            MatchOutput::Err(err) => {
                MatchOutput::Ok(format!("not({})", err))
            }
        }
    }
}

impl <Inner: Debug> Debug for NotMatcher<Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("not").field(&self.0).finish()
    }
}

pub struct EqMatcher<T: Eq>(T);

pub fn eq<T: Eq>(value: T) -> EqMatcher<T> {
    EqMatcher(value)
}

impl <T: Eq> Matcher<T> for EqMatcher<T> {
    fn matches(&self, value: T) -> MatchOutput {
        if self.0 == value {
            MatchOutput::Ok("Values equal".to_string())
        } else {
            MatchOutput::Err(format!("Expected {} to be equal to {}", 0, 0))
        }
    }
}

impl <T: Eq + Debug> Debug for EqMatcher<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("eq").field(&self.0).finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_anything() {
        assert_that(123, anything());
    }

    #[test]
    fn test_eq() {
        assert_that(123, eq(123));
    }

    #[test]
    fn test_not_eq() {
        assert_that(123, not(eq(456)));
    }
}
