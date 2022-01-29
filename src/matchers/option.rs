use std::fmt::Debug;
use super::{Matcher, MatchOutput};

pub struct SomeMatcher<Inner: Debug>(Inner);

pub fn is_some<Inner: Debug>(matcher: Inner) -> SomeMatcher<Inner> {
    SomeMatcher(matcher)
}

impl <Inner: Debug, T> Matcher<Option<T>> for SomeMatcher<Inner>
where
    Inner: Matcher<T>
{
    fn matches(&self, t: Option<T>) -> MatchOutput {
        match t {
            Some(value) => {
                self.0.matches(value).wrap_with_ok("Some(", ")")
            },
            None => {
                MatchOutput::expected_found("Some(_)".to_string(), "None".to_string())
            }
        }
    }
}

impl <Inner: Debug> Debug for SomeMatcher<Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Some").field(&self.0).finish()
    }
}

pub struct NoneMatcher;

pub fn is_none() -> NoneMatcher {
    NoneMatcher
}

impl <T> Matcher<Option<T>> for NoneMatcher {
    fn matches(&self, t: Option<T>) -> MatchOutput {
        match t {
            Some(_) => {
                MatchOutput::expected_found("None".to_string(), "Some(...)".to_string())
            },
            None => {
                MatchOutput::Ok("None".to_string())
            }
        }
    }
}

impl Debug for NoneMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("None").finish()
    }
}