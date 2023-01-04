use dye_from::HighlightJs;

#[test]
fn test() {
    let parser = HighlightJs::builtin();
    parser.parse_html(include_str!("highlightjs.in.html")).unwrap();
}