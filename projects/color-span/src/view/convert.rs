use super::*;

impl Debug for ColorView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ColorView")
            .field(&self.text())
            // .field(&self.span.get_info().iter().map(|v| v.clone().unwrap_or_default()))
            .finish()
    }
}

impl<T> From<T> for ColorView
where
    T: Into<String>,
{
    fn from(s: T) -> Self {
        ColorView::new(s)
    }
}
