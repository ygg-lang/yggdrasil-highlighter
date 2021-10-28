use std::{fmt::Debug, ops::Range};

use serde::{Deserialize, Serialize};

mod convert;
pub mod iter;

/// Save the given code and store additional information on each character
///
/// # Examples
///
/// ```
/// use code_span::CodeView;
/// CodeView::from("public static class MyClass {}");
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeView<T> {
    text: String,
    info: Vec<Option<T>>,
}

/// Each character of these code stores the same information
///
/// # Examples
///
/// ```
/// use code_span::CodeSpan;
/// CodeSpan { text: "public".to_string(), info: Some("keyword".to_string()) };
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeSpan<T> {
    /// text
    pub text: String,
    /// info
    pub info: Option<T>,
}

impl<T> CodeView<T> {
    /// Create a piece of code with no information
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::CodeView;
    /// CodeView::blank("public static class MyClass {}");
    /// ```
    pub fn blank(text: impl Into<String>) -> Self
    where
        T: Clone,
    {
        let text = text.into();
        let count = text.chars().count();
        Self { text, info: vec![None; count] }
    }
    /// Create a piece of code with specified information
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::CodeView;
    /// CodeView::new("public".to_string(), vec![Some("keyword"); 6]);
    /// ```
    pub fn new(text: String, info: Vec<Option<T>>) -> Self {
        assert_eq!(text.chars().count(), info.len());
        Self { text, info }
    }
    /// Get original code
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::CodeView;
    /// let view = CodeView::blank("public static class MyClass {}");
    /// assert_eq!(view.text(), "public static class MyClass {}");
    /// ```
    #[inline]
    pub fn get_text(&self) -> &str {
        &self.text
    }
    /// Modify the original code
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::CodeView;
    /// let mut view = CodeView::blank("public static class MyClass {}");
    /// view.set_text("private static class MyClass {}");
    /// ```
    #[inline]
    pub fn mut_text(&mut self) -> &mut String {
        &mut self.text
    }
    /// Get current information
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::CodeView;
    /// let view = CodeView::blank("public static class MyClass {}");
    /// assert_eq!(view.get_info(), vec![None; 30]);
    /// ```
    #[inline]
    pub fn get_info(&self) -> &[Option<T>] {
        &self.info
    }
    /// Modify current information
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::CodeView;
    /// let mut view = CodeView::blank("public");
    /// view.mut_info().for_each(|v| v = Some("keyword"));
    /// ```
    #[inline]
    pub fn mut_info(&mut self) -> &mut [Option<T>] {
        &mut self.info
    }

    /// Mark the information of a piece of code according to the character range
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::CodeView;
    /// let mut view = CodeView::blank("public static class MyClass {}");
    /// view.mark_position(0, 6, Some("keyword"));
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
    /// Mark the information of a piece of code according to the byte range
    ///
    /// # Examples
    ///
    /// ```
    /// use code_span::CodeView;
    /// let mut view = CodeView::blank("public static class MyClass {}");
    /// view.mark_offset(0, 6, Some("keyword"));
    /// ```
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
