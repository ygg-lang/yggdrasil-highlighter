use super::*;

impl Debug for ColorView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ColorView").field(&self.text()).finish()
    }
}

impl From<&str> for ColorView {
    fn from(s: &str) -> Self {
        ColorView::new(s)
    }
}

impl From<String> for ColorView {
    fn from(s: String) -> Self {
        ColorView::new(&s)
    }
}
