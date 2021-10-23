use indexmap::IndexSet;

use crate::{Palette, TextView};

impl Default for Palette {
    fn default() -> Self {
        let mut classes = IndexSet::default();
        classes.insert("".to_string());
        Self { classes, text: TextView::new("") }
    }
}

impl<T> From<T> for Palette
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Palette::default().with_text(value.as_ref())
    }
}
