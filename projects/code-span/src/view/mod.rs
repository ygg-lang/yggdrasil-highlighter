use std::ops::Range;

use serde::{Deserialize, Serialize};

mod convert;
pub mod iter;

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
/// use code_span::CodeView;
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeView<T> {
    text: String,
    info: Vec<Option<T>>,
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
/// use code_span::CodeView;
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeSpan<T> {
    /// text
    pub text: String,
    /// info
    pub info: Option<T>,
}

impl<T> CodeView<T> {
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
    /// use code_span::CodeView;
    /// ```
    pub fn blank(text: impl Into<String>) -> Self
    where
        T: Clone,
    {
        let text = text.into();
        let count = text.chars().count();
        Self { text, info: vec![None; count] }
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
    /// use code_span::CodeView;
    /// ```
    pub fn new(text: String, info: Vec<Option<T>>) -> Self {
        assert_eq!(text.chars().count(), info.len());
        Self { text, info }
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
    /// use code_span::CodeView;
    /// ```
    #[inline]
    pub fn get_text(&self) -> &str {
        &self.text
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
    /// use code_span::CodeView;
    /// ```
    #[inline]
    pub fn mut_text(&mut self) -> &mut String {
        &mut self.text
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
    /// use code_span::CodeView;
    /// ```
    #[inline]
    pub fn get_info(&self) -> &[Option<T>] {
        &self.info
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
    /// use code_span::CodeView;
    /// ```
    #[inline]
    pub fn mut_info(&mut self) -> &mut [Option<T>] {
        &mut self.info
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
    /// use code_span::CodeView;
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
    /// use code_span::CodeView;
    /// ```
    #[inline]
    pub fn mark_offset(&mut self, start: usize, end: usize, info: Option<T>)
    where
        T: Clone,
    {
        debug_assert!(start <= end);
        let end = self.text.len().min(end);
        let start = self.text[..start].chars().count();
        let end = start + self.text[start..end].chars().count();
        self.mark_position(start, end, info)
    }
}
