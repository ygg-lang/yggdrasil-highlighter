use std::fmt::{Debug, Formatter};

use super::*;

impl<T> Debug for CodeView<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CodeSpan").field(&self.text).finish()
    }
}

impl<T> From<&str> for CodeView<T>
where
    T: Clone,
{
    fn from(s: &str) -> Self {
        CodeView::empty(s)
    }
}

impl<T> From<String> for CodeView<T>
where
    T: Clone,
{
    fn from(s: String) -> Self {
        CodeView::empty(s)
    }
}
