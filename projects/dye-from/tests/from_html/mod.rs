use std::collections::{BTreeMap, BTreeSet};

use diagnostic_quick::QResult;
use tl::{HTMLTag, Node, parse, Parser, ParserOptions};

use color_span::ClassPalette;

#[test]
fn test() {
    let parser = HighlightJs::builtin();
    parser.parse_html(include_str!("highlightjs.in.html")).unwrap();
}

#[derive(Debug, Default)]
pub struct HighlightJs {
    drop_map: BTreeSet<String>,
    class_map: BTreeMap<String, String>,
}

impl HighlightJs {
    pub fn parse_html(&self, html: &str) -> QResult {
        let dom = parse(html, ParserOptions::default())?;
        let mut out = vec![];
        let parser = dom.parser();
        for block in dom.get_elements_by_class_name("hljs") {
            let node_parser = NodeContext {
                config: self,
                parser,
                language: "".to_string(),
                code: "".to_string(),
                span: vec![],
            };
            if let Some(s) = node_parser.parse_node(block.get(parser)) {
                out.push(s.as_palette())
            }
        }
        println!("{:#?}", out);
        Ok(())
    }
}

#[derive(Debug)]
pub struct NodeContext<'i> {
    config: &'i HighlightJs,
    parser: &'i Parser<'i>,
    language: String,
    code: String,
    span: Vec<ClassSpan>,
}

#[derive(Debug)]
pub struct ClassSpan {
    class: String,
    start: usize,
    end: usize,
}

impl NodeContext<'_> {
    #[inline]
    fn parse_node(mut self, node: Option<&Node>) -> Option<Self> {
        let tag = node?.as_tag()?;
        self.check_code(tag);
        self.extract_language(tag);
        self.visit_children(tag);
        Some(self)
    }
    fn check_code(&mut self, tag: &HTMLTag) -> Option<()> {
        if !tag.name().eq("code") {}
        None
    }
    fn extract_language(&mut self, tag: &HTMLTag) {
        for get_class in get_class(tag).split(' ') {
            if get_class.starts_with("language-") {
                self.language = get_class[9..].to_string();
                return;
            }
        }
    }
    fn visit_children(&mut self, tag: &HTMLTag) -> Option<()> {
        let mut l_ptr = 0;
        for child in tag.children().all(self.parser) {
            match child {
                Node::Tag(v) => {
                    self.visit_child_tag(v, "", l_ptr);
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
    fn visit_child_tag(&mut self, tag: &HTMLTag, parent_class: &str, mut l_ptr: usize) -> Option<()> {
        let class = self.split_class(get_class(tag));
        for child in tag.children().all(self.parser) {
            match child {
                Node::Tag(v) => {
                    self.visit_child_tag(v, &class, l_ptr);
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

    fn split_class(&self, classes: &str) -> String {
        if self.config.drop_map.contains(classes) {
            return "".to_string();
        }
        for class in classes.split(' ') {
            if class.starts_with("language") {
                return String::new();
            }

            if let Some(s) = self.config.class_map.get(class) { return s.to_string(); }
        }
        println!("unknown classes: {}", classes);
        String::new()
    }
}

impl NodeContext<'_> {
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

impl HighlightJs {
    #[inline]
    pub fn builtin() -> Self {
        let mut out = Self::default();
        out.add_builtin();
        out
    }
    pub fn add_builtin(&mut self) {
        self.insert_drop("hljs-title");
        self.insert_drop("hljs-pattern-match"); // code span 不标记块

        // 关键词
        self.insert_rule("hljs-meta", "meta");
        self.insert_rule("hljs-keyword", "keyword");
        self.insert_rule("hljs-punctuation", "punctuation");
        self.insert_rule("hljs-operator", "operator");

        self.insert_rule("hljs-section", "markup-title");
        self.insert_rule("hljs-bullet", "markup-bullet");



        // literal
        self.insert_rule("hljs-module-access", "module"); // 带 . 的 module path
        self.insert_rule("hljs-module", "module");

        self.insert_rule("hljs-number", "number");
        self.insert_rule("hljs-symbol", "identifier"); // 不明, 啥都可以是 symbol
        self.insert_rule("hljs-identifier", "identifier");
        self.insert_rule("hljs-built_in", "builtin");
        self.insert_rule("hljs-attribute", "attribute"); // 实际表示键值对的键
        self.insert_rule("hljs-attr", "attribute");
        self.insert_rule("hljs-variable", "variable"); // 特制本地变量
        self.insert_rule("hljs-params", "argument"); // 实际上表示形参

        self.insert_rule("hljs-constructor", "variant"); // 实际上表示枚举构造函数

        self.insert_rule("hljs-tag", ""); // 独指 XML/HTML 的标签
        self.insert_rule("hljs-name", "class"); // 独指 XML/HTML 的标签值, 以及某些变量名

        //  hljs-subst 插值变量
        self.insert_rule("hljs-property", "property"); // 对象属性
        self.insert_rule("hljs-function", "function");
        self.insert_rule("function_", "function");

        // typing
        self.insert_rule("hljs-type", "type");
        self.insert_rule("hljs-class", "class");
        self.insert_rule("class_", "class");


        // string-like
        self.insert_rule("hljs-string", "string");
        self.insert_rule("escape_", "escape");
        self.insert_rule("hljs-literal", "literal");
        self.insert_rule("hljs-quote", "literal"); // 指代大段文本, 没有 '' 包裹的那种
        self.insert_rule("hljs-link", "link"); // 指代 url, uri, email 等等
        self.insert_rule("hljs-comment", "comment");
    }
    pub fn insert_rule(&mut self, js: impl Into<String>, class: impl Into<String>) {
        self.class_map.insert(js.into(), class.into());
    }
    pub fn insert_drop(&mut self, js: impl Into<String>) {
        self.drop_map.insert(js.into());
    }
}

