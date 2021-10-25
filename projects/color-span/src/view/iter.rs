use internship::IStr;

use code_span::CodeViewIter;

use crate::{view::ColorSpan, ColorView};
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
#[derive(Debug)]
pub struct ColorViewIter<'i> {
    iter: CodeViewIter<'i, IStr>,
}

impl<'i> IntoIterator for &'i ColorView {
    type Item = ColorSpan;
    type IntoIter = ColorViewIter<'i>;

    fn into_iter(self) -> Self::IntoIter {
        ColorViewIter { iter: self.span.into_iter() }
    }
}

impl Iterator for ColorViewIter<'_> {
    type Item = ColorSpan;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        Some(ColorSpan { color: item.info.unwrap_or_default(), text: item.text })
    }
}
