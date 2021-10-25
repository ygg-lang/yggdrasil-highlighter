use std::{iter::Zip, mem::take, slice::Iter, str::Chars};

use crate::{view::CodeView, CodeSpan};

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
#[derive(Debug)]
pub struct CodeViewIter<'i, T> {
    iter: Zip<Iter<'i, Option<T>>, Chars<'i>>,
    current: Option<T>,
    buffer: String,
    run_out: bool,
}

impl<'i, T> IntoIterator for &'i CodeView<T>
where
    T: Clone + PartialEq,
{
    type Item = CodeSpan<T>;
    type IntoIter = CodeViewIter<'i, T>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.info.iter().zip(self.text.chars());
        CodeViewIter { run_out: false, current: None, iter, buffer: "".to_string() }
    }
}

impl<'i, T> Iterator for CodeViewIter<'i, T>
where
    T: Clone + PartialEq,
{
    type Item = CodeSpan<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.run_out {
            return None;
        }
        while let Some(this) = self.iter.next() {
            if self.current.eq(this.0) {
                self.buffer.push(this.1);
                continue;
            }
            else {
                let out = self.pop_span();
                self.buffer.push(this.1);
                self.current = this.0.clone();
                if out.text.is_empty() {
                    continue;
                }
                else {
                    return Some(out);
                }
            }
        }
        self.run_out = true;
        Some(self.pop_span())
    }
}

impl<'i, T> CodeViewIter<'i, T>
where
    T: Clone,
{
    #[inline]
    fn pop_span(&mut self) -> CodeSpan<T> {
        CodeSpan { text: take(&mut self.buffer), info: self.current.clone() }
    }
}
