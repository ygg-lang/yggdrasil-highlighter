use std::fmt::{Debug, Formatter};

use super::*;

impl<T> Debug for CodeSpan<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CodeSpan").field(&self.text).finish()
    }
}

impl<T> From<&str> for CodeSpan<T>
where
    T: Clone,
{
    fn from(s: &str) -> Self {
        CodeSpan::new(s)
    }
}

impl<T> From<String> for CodeSpan<T>
where
    T: Clone,
{
    fn from(s: String) -> Self {
        CodeSpan::new(s)
    }
}
