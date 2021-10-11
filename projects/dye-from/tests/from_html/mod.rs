use tl::{Attributes, HTMLTag, Node};

#[test]
fn test() {
    let dom = tl::parse(include_str!("highlightjs.in.html"), tl::ParserOptions::default()).unwrap();
    let mut out = vec![];
    let parser = dom.parser();
    for block in dom.get_elements_by_class_name("hljs") {
        if let Some(s) = HighlightJs::parse_node(block.get(parser)) { out.push(s) }
    }
}

impl HighlightJs {
    #[inline]
    fn parse_node(node: Option<&Node>) -> Option<Self> {
        let tag = node?.as_tag()?;
        let mut out = HighlightJs::default();
        out.check_code(tag);
        out.extract_language(tag.attributes());

        for child in tag.children() {
            match child {
                Node::Text(text) => out.text.push_str(text),
                Node::Tag(tag) => {
                    let tag = tag.as_tag()?;
                    if tag.name() == "span" {
                        out.parse_span(tag);
                    }
                }
                _ => {}
            }
        }


        println!("{:#?}", out.extract_language(attr));

        todo!()
    }
    fn check_code(&mut self, tag: &HTMLTag) -> Option<()> {
        let is_code = tag.name().eq("code");
        println!("is_code: {}", is_code);
        None
    }

    fn extract_language(&mut self, attr: &Attributes) -> Option<()> {
        let class = attr.get("class")??.as_utf8_str();


        println!("{:#?}", class);
        None
    }
}


#[derive(Debug, Default)]
pub struct HighlightJs {
    pub language: String,
}