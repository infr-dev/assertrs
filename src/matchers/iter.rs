use std::{fmt::Debug};
use super::{Matcher, MatchOutput};

pub struct AllMatcher<Inner>(Inner);

pub fn all<Inner>(matcher: Inner) -> AllMatcher<Inner> {
    AllMatcher(matcher)
}

impl <Inner, T, I> Matcher<I> for AllMatcher<Inner>
where
    Inner: Matcher<T>,
    I: IntoIterator<Item = T>
{
    fn matches(&self, value: I) -> MatchOutput {
        for i in value.into_iter() {
            if let MatchOutput::Err(err) = self.0.matches(i) {
                return MatchOutput::Err(err);
            }
        }

        MatchOutput::Ok("All matches".to_string())
    }
}

impl <Inner: Debug> Debug for AllMatcher<Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("all").field(&self.0).finish()
    }
}

pub struct AnyMatcher<Inner>(Inner);

pub fn any<Inner>(matcher: Inner) -> AnyMatcher<Inner> {
    AnyMatcher(matcher)
}

impl <Inner, T, I> Matcher<I> for AnyMatcher<Inner>
where
    Inner: Matcher<T>,
    I: IntoIterator<Item = T>
{
    fn matches(&self, value: I) -> MatchOutput {
        for i in value.into_iter() {
            if let MatchOutput::Ok(err) = self.0.matches(i) {
                return MatchOutput::Ok(err);
            }
        }

        MatchOutput::Err("None matches".to_string())
    }
}

impl <Inner: Debug> Debug for AnyMatcher<Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("any").field(&self.0).finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_all() {
        assert_that(&[10, 20, 30], all(anything()));
        assert_that(&[10, 20, 30], all(not(eq(&100))));
    }

    #[test]
    fn test_any() {
        assert_that(vec![10, 20, 30], any(eq(20)));
    }
}