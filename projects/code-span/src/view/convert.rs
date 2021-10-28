use super::*;

impl<T> From<&str> for CodeView<T>
where
    T: Clone,
{
    fn from(s: &str) -> Self {
        CodeView::blank(s)
    }
}

impl<T> From<String> for CodeView<T>
where
    T: Clone,
{
    fn from(s: String) -> Self {
        CodeView::blank(s)
    }
}
