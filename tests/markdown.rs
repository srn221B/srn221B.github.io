use leptos_post::markdown::{TocEntry, render_markdown};

fn entry(level: u8, id: &str, text: &str) -> TocEntry {
    TocEntry {
        level,
        id: id.to_string(),
        text: text.to_string(),
    }
}

#[test]
fn toc_lists_headings_in_order() {
    let rendered = render_markdown("# A\n\ntext\n\n## B c\n\n### D\n");
    assert_eq!(
        rendered.toc,
        [
            entry(1, "a", "A"),
            entry(2, "b-c", "B c"),
            entry(3, "d", "D")
        ]
    );
}

#[test]
fn toc_ids_match_rendered_html_with_duplicates() {
    let rendered = render_markdown("## Same\n\n## Same\n\n## 日本語\n");
    for toc_entry in &rendered.toc {
        assert!(
            rendered.html.contains(&format!("id=\"{}\"", toc_entry.id)),
            "id {:?} not in html: {}",
            toc_entry.id,
            rendered.html
        );
    }
    assert_eq!(rendered.toc[1].id, "same-1");
}

#[test]
fn toc_is_empty_without_headings() {
    assert!(render_markdown("just text").toc.is_empty());
}
