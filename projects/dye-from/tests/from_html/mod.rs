use tl::{HTMLTag, Node, Parser};

use color_span::ClassPalette;

#[test]
fn test() {
    let dom = tl::parse(include_str!("highlightjs.in.html"), tl::ParserOptions::default()).unwrap();
    let mut out = vec![];
    let parser = dom.parser();
    for block in dom.get_elements_by_class_name("hljs") {
        if let Some(s) = HighlightJs::parse_node(block.get(parser), parser) {
            out.push(s.as_palette())
        }
    }
    println!("{:#?}", out);
}

#[derive(Debug, Default)]
pub struct HighlightJs {
    pub language: String,
    pub code: String,
    pub span: Vec<ClassSpan>,
}

#[derive(Debug)]
pub struct ClassSpan {
    class: String,
    start: usize,
    end: usize,
}

impl HighlightJs {
    #[inline]
    fn parse_node(node: Option<&Node>, parser: &Parser) -> Option<Self> {
        let tag = node?.as_tag()?;
        let mut out = HighlightJs::default();
        out.check_code(tag);
        out.extract_language(tag);
        out.visit_children(tag, parser);
        Some(out)
    }
    fn check_code(&mut self, tag: &HTMLTag) -> Option<()> {
        if !tag.name().eq("code") {}
        None
    }
    fn extract_language(&mut self, tag: &HTMLTag) -> Option<()> {
        let class = get_class(tag);
        println!("{:#?}", class);
        None
    }
    fn visit_children(&mut self, tag: &HTMLTag, parser: &Parser) -> Option<()> {
        let mut l_ptr = 0;
        for child in tag.children().all(parser) {
            match child {
                Node::Tag(v) => {
                    self.visit_child_tag(v, parser, "", l_ptr);
                }
                Node::Raw(v) => {
                    let text = v.as_utf8_str();
                    l_ptr += text.len();
                    self.code.push_str(&v.as_utf8_str());
                }
                // drop comment
                Node::Comment(_) => {}
            }
        }
        None
    }
    fn visit_child_tag(&mut self, tag: &HTMLTag, parser: &Parser, parent_class: &str, mut l_ptr: usize) -> Option<()> {
        let class = split_class(get_class(tag));
        for child in tag.children().all(parser) {
            match child {
                Node::Tag(v) => {
                    self.visit_child_tag(v, parser, class, l_ptr);
                }
                Node::Raw(v) => {
                    let text = v.as_utf8_str();
                    let r_ptr = l_ptr + text.len();
                    self.span.push(ClassSpan {
                        class: parent_class.to_string(),
                        start: l_ptr,
                        end: r_ptr,
                    });
                    self.code.push_str(&v.as_utf8_str());
                    l_ptr = r_ptr;
                }
                // drop comment
                Node::Comment(_) => {}
            }
        }
        None
    }
}

impl HighlightJs {
    pub fn as_palette(&self) -> ClassPalette {
        let mut palette = ClassPalette::from(&self.code);
        for span in &self.span {
            palette.dye(span.start, span.end, &span.class).ok();
        }
        palette
    }
}

#[inline]
fn get_class<'i>(tag: &'i HTMLTag<'i>) -> &'i str {
    match tag.attributes().class().and_then(|v| v.try_as_utf8_str()) {
        Some(s) => { s }
        None => { "" }
    }
}


fn split_class(classes: &str) -> &str {
    for class in classes.split(' ') {
        return match class {
            "hljs-meta" => "meta",
            "hljs-keyword" => "keyword",
            "hljs-punctuation" => "punctuation",
            "hljs-type" => "type",
            "hljs-symbol" => "identifier",
            "hljs-string" => "string",
            "hljs-built_in" => "builtin",
            _ => {
                println!("unknown class: {}", class);
                class
            }
        };
    }
    return "";
}