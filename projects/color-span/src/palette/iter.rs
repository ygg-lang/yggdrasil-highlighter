use crate::{view::iter::TextColorIter, Palette};
use indexmap::IndexSet;

/// Iter type for ClassPalette
#[derive(Debug)]
pub struct ClassPaletteIter<'i> {
    text: TextColorIter<'i>,
}

impl<'i> IntoIterator for &'i Palette {
    type Item = (String, String);
    type IntoIter = ClassPaletteIter<'i>;

    fn into_iter(self) -> Self::IntoIter {
        ClassPaletteIter { text: self.into_iter() }
    }
}

impl Iterator for ClassPaletteIter<'_> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let Colored { value, color } = self.text.next()?;
        let class = match self.map.get_index(color as usize) {
            Some(s) => s.to_string(),
            None => String::new(),
        };
        Some((class, value))
    }
}
