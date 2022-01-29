mod iter;
pub use iter::*;

mod misc;
pub use misc::*;

mod option;
pub use option::*;

pub trait Matcher<T> {
    fn matches(&self, t: T) -> MatchOutput;
}

pub enum MatchOutput {
    Ok(String),
    Err(String),
}

impl MatchOutput {
    pub fn wrap_with_ok(self, pre: &str, post: &str) -> MatchOutput {
        match self {
            MatchOutput::Ok(ok) => {
                MatchOutput::Ok(format!("{}{}{}", pre, ok, post))
            },
            MatchOutput::Err(err) => {
                MatchOutput::Err(format!("{}{}{}", pre, err, post))
            }
        }
    }

    pub fn wrap_with_err(self, pre: &str, post: &str) -> MatchOutput {
        match self {
            MatchOutput::Ok(ok) => {
                MatchOutput::Ok(format!("{}{}{}", pre, ok, post))
            },
            MatchOutput::Err(err) => {
                MatchOutput::Err(format!("{}{}{}", pre, err, post))
            }
        }
    }

    pub fn expected_found(expected: String, found: String) -> MatchOutput {
        MatchOutput::Err(format!("Expected {} but found {}", expected, found))
    }
}