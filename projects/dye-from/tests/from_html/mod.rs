use tl::{Attributes, HTMLTag, Node, Parser};

#[test]
fn test() {
    let dom = tl::parse(include_str!("highlightjs.in.html"), tl::ParserOptions::default()).unwrap();
    let mut out = vec![];
    let parser = dom.parser();
    for block in dom.get_elements_by_class_name("hljs") {
        if let Some(s) = HighlightJs::parse_node(block.get(parser), parser) { out.push(s) }
    }
}

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
        out.check_code(tag)?;
        out.extract_language(tag.attributes())?;
        let mut l_ptr = 0;
        for child in tag.children().all(parser) {
            out.visit_child(child, l_ptr);
        }
        Some(out)
    }
    fn check_code(&mut self, tag: &HTMLTag) -> Option<()> {
        if tag.name().eq("code") {}
        Some(())
    }

    fn extract_language(&mut self, attr: &Attributes) -> Option<()> {
        let class = attr.get("class")??.as_utf8_str();


        println!("{:#?}", class);
        Some(())
    }

    fn visit_child(&mut self, node: &Node, mut l_ptr: usize) -> Option<()> {
        match node {
            Node::Tag(v) => {
                println!("tag: {:?}", v);
            }
            Node::Raw(v) => {
                let text = v.as_utf8_str();
                l_ptr += text.len();
                self.code.push_str(&v.as_utf8_str());
            }
            Node::Comment(_) => {}
        }
        Some(())
    }
}


#[derive(Debug, Default)]
pub struct HighlightJs {
    pub language: String,
    pub code: String,
}