use std::ops::Range;

use lsp_document::{IndexedText, TextMap};

// mod iter;

/// # Arguments
///
/// * `text`:
/// * `default`:
///
/// returns: TextView<T>
///
/// # Examples
///
/// ```
/// use code_span::CodeSpan;
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeSpan<'i, T> {
    /// Raw character
    map: IndexedText<&'i str>,
    /// Information
    info: Vec<Option<T>>,
}

impl<'i, T> CodeSpan<'i, T> {
    /// # Arguments
    ///
    /// * `text`:
    /// * `default`:
    ///
    /// returns: TextView<T>
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::CodeSpan;
    /// ```
    pub fn new(text: &'i str) -> Self
    where
        T: Clone,
    {
        let count = text.chars().count();
        let map = IndexedText::new(text);
        Self { map, info: vec![None; count] }
    }
    /// # Arguments
    ///
    /// * `text`:
    /// * `default`:
    ///
    /// returns: TextView<T>
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::TextView;
    /// ```
    pub fn text(&self) -> &str {
        self.map.text()
    }
    /// # Arguments
    ///
    /// * `start`:
    /// * `end`:
    /// * `info`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::TextView;
    /// ```
    pub fn mark_position(&mut self, start: usize, end: usize, info: Option<T>)
    where
        T: Clone,
    {
        debug_assert!(start <= end);
        let end = self.info.len().min(end);
        let items = unsafe { self.info.get_unchecked_mut(Range { start, end }) };
        for item in items {
            *item = info.clone()
        }
    }
    /// # Arguments
    ///
    /// * `start`:
    /// * `end`:
    /// * `info`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::TextView;
    /// ```
    pub fn mark_offset(&mut self, start: usize, end: usize, info: Option<T>)
    where
        T: Clone,
    {
        debug_assert!(start <= end);
        let end = self.info.len().min(end);
        let items = unsafe { self.info.get_unchecked_mut(Range { start, end }) };
        for item in items {
            *item = info.clone()
        }
    }
}
