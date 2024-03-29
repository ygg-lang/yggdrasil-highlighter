use std::fmt::{Debug, Formatter};

use internship::IStr;
use serde::{Deserialize, Serialize};

use code_span::CodeView;

mod convert;
mod der;
pub mod iter;
mod ser;

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
#[derive(Clone, Eq, PartialEq)]
pub struct ColorView {
    span: CodeView<IStr>,
}

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
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ColorSpan {
    /// content of the code span
    pub text: String,
    /// color class of the code span
    pub color: IStr,
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
        Self { span: CodeView::blank(text) }
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
        self.span.get_text()
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
