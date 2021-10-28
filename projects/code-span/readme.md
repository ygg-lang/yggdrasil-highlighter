Code Span
=========

Add additional information to your code spans.

```rust
fn main() {
    use code_span::CodeView;
    let mut view = CodeView::blank("public static class MyClass {}");
    view.mark_offset(0, 6, Some("keyword"));
    view.mark_offset(7, 13, Some("keyword"));
}
```