#![feature(generic_associated_types)]

mod matchers;
pub use matchers::*;

pub fn assert_that<T>(x: T, matcher: impl Matcher<T>) {
    if let MatchOutput::Err(err) = matcher.matches(x) {
        panic!("{:?}", err);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_that(Some(123), anything());
        assert_that(Some(123), is_some(anything()));
    }
}
