use tl::{HTMLTag, Node};

#[test]
fn test() {
    let dom = tl::parse(include_str!("highlightjs.in.html"), tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    for block in dom.get_elements_by_class_name("hljs") {
        HighlightJs::default();
        parse_node(block.get(parser));
    }
}

impl HighlightJs {
    #[inline]
    fn parse_node(node: Option<&Node>) -> Option<Self> {
        let tag = node?.as_tag()?;
        let mut out = HighlightJs::default();

        if tag.name().eq("code") {
            return None
        }
        let attr = tag.attributes();
        attr.get("class")?;



        println!("{:#?}", tag);
        Some(true)
    }
}



#[derive(Debug, Default)]
pub struct HighlightJs {
    pub language: String,
}