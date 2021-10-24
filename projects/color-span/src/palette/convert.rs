use indexmap::IndexSet;

use crate::Palette;

impl Default for Palette {
    fn default() -> Self {
        Self { classes: IndexSet::from_iter(vec!["".to_string()]) }
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
