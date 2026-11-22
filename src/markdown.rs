use comrak::{
    Anchorizer, Arena, Options, format_html_with_plugins, nodes::NodeValue, options::Plugins,
    parse_document, plugins::syntect::SyntectAdapter,
};

pub struct Rendered {
    pub html: String,
    pub toc: Vec<TocEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TocEntry {
    pub level: u8,
    pub id: String,
    pub text: String,
}

pub fn render_markdown(md: &str) -> Rendered {
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.header_id_prefix = Some(String::new());
    options.render.r#unsafe = true; // 記事中の生 HTML をそのまま通す

    let adapter = SyntectAdapter::new(Some("base16-ocean.dark"));
    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let arena = Arena::new();
    let root = parse_document(&arena, md, &options);

    let mut anchorizer = Anchorizer::new();
    let toc = root
        .descendants()
        .filter_map(|node| match &node.data.borrow().value {
            NodeValue::Heading(heading) => {
                let text = node.collect_text();
                let id = anchorizer.anchorize(&text);
                Some(TocEntry {
                    level: heading.level,
                    id,
                    text,
                })
            }
            _ => None,
        })
        .collect();

    let mut html = String::new();
    format_html_with_plugins(root, &options, &mut html, &plugins)
        .expect("String への書き込みは失敗しない");

    Rendered { html, toc }
}
