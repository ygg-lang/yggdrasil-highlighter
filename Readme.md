Dye-ware Highlighter
====================

Add additional information to your code spans.

```rust
fn main() {
    use code_view::ColorView;
    let mut view = ColorView::from("public static class G {}");
    view.dye_offset(0, 6, "keyword");
    view.dye_offset(7, 13, "keyword");
}
```