use indexmap::IndexSet;

use crate::{ClassPalette, TextView};

impl Default for ClassPalette {
    fn default() -> Self {
        let mut classes = IndexSet::default();
        classes.insert("".to_string());
        Self { classes, text: TextView::new("") }
    }
}

impl<T> From<T> for ClassPalette
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        ClassPalette::default().with_text(value.as_ref())
    }
}
