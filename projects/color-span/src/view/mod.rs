use std::fmt::Debug;

use internship::IStr;

use arc_interner::ArcIntern;
use code_span::CodeSpan;

// mod convert;
// mod der;
// pub mod iter;
// mod ser;

/// Write color palette into html
///
/// **Support 255 color at most**.
///
/// # Arguments
///
/// * `w`:
///
/// returns: Result<(), Error>
///
/// # Examples
///
/// ```
/// use color_span::ColorClass;
/// ```
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ColorView {
    span: CodeSpan<IStr>,
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ColorSpan {
    color: IStr,
    text: String,
}

impl ColorView {
    /// # Arguments
    ///
    /// * `text`:
    ///
    /// returns: TextColorView
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::ColorView;
    /// ```
    #[inline]
    pub fn new(text: impl Into<String>) -> ColorView {
        Self { span: CodeSpan::new(text) }
    }
    /// Color the text in the range of `start`..`end` to given color name
    ///
    /// # Arguments
    ///
    /// * `start`: start offset
    /// * `end`: end offset
    /// * `color`: color name
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::ColorView;
    /// ```
    pub fn text(&self) -> &str {
        self.span.text()
    }
    /// Color the text in the range of `start`..`end` to given color name
    ///
    /// # Arguments
    ///
    /// * `start`: start offset
    /// * `end`: end offset
    /// * `color`: color name
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::ColorView;
    /// ```
    pub fn dye_position(&mut self, start: usize, end: usize, color: &str) {
        let color = match color {
            "" => None,
            _ => Some(IStr::new(color)),
        };
        self.span.mark_position(start, end, color)
    }
    /// Color the text in the range of `start`..`end` to given color name
    ///
    /// # Arguments
    ///
    /// * `start`: start offset
    /// * `end`: end offset
    /// * `color`: color name
    ///
    /// # Examples
    ///
    /// ```
    /// use color_span::ColorView;
    /// ```
    pub fn dye_offset(&mut self, start: usize, end: usize, color: &str) {
        let color = match color {
            "" => None,
            _ => Some(IStr::new(color)),
        };
        self.span.mark_offset(start, end, color)
    }
}
