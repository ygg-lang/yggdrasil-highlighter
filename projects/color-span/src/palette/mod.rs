use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use crate::{ColorSpanError, TextView};

mod convert;
mod iter;

/// Get or insert
///
/// # Arguments
///
/// * `key`:
///
/// returns: Result<u8, ColorSpanError>
///
/// # Examples
///
/// ```
/// use color_span::TextView;
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClassPalette {
    classes: IndexSet<String>,
    text: TextView,
}

impl ClassPalette {
    /// Get the current uncolored text
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::ClassPalette;
    /// let mut view = ClassPalette::from("public static class G {}");
    /// view.get_text();
    /// ```
    #[inline]
    pub fn get_text(&self) -> String {
        self.text.text()
    }
    /// Set the current text
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::ClassPalette;
    /// let mut view = ClassPalette::from("public static class G {}");
    /// view.set_text("let mut a = 0");
    /// view.get_text();
    /// ```
    #[inline]
    pub fn set_text(&mut self, text: &str) {
        self.text = TextView::new(text);
    }
    /// Create a new [`ClassPalette`] with given text
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::ClassPalette;
    /// let view = ClassPalette::from("public static class G {}").with_text("let mut a = 0");
    /// view.get_text();
    /// ```
    #[inline]
    pub fn with_text(self, text: &str) -> Self {
        Self { classes: self.classes, text: TextView::new(text) }
    }
    /// Tint the texts in the range to the specified color
    ///
    /// # Arguments
    ///
    /// * `start`: start offset of the span
    /// * `end`: end offset of the span
    /// * `color`: color to tint the span
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::ClassPalette;
    /// let mut view = ClassPalette::from("public static class G {}");
    /// view.dye(0, 6, "keyword").unwrap();
    /// ```
    pub fn dye(&mut self, start: usize, end: usize, color: &str) -> Result<u32, ColorSpanError> {
        let index = match self.classes.get_full(color) {
            Some(s) => s.0,
            None => self.classes.insert_full(color.to_string()).0,
        };
        let index = if index <= 2047 { index as u32 } else { Err(ColorSpanError::TooMuchColors)? };
        self.text.dye(start, end, index)?;
        Ok(index)
    }
}
