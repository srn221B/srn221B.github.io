use leptos_post::post::{FrontMatter, Post, group_by_year, posts_by_tag, published};

const SRC: &str = "---\ntitle: t\npublishedAt: 2026-01-01\nisPublish: true\ntags: [a]\n---\n# h\n";

#[test]
fn parses_front_matter_and_body() {
    let post = Post::parse("s", SRC).unwrap();
    assert_eq!(post.meta.title, "t");
    assert_eq!(post.meta.tags, ["a"]);
    assert!(post.meta.is_published());
    assert!(post.body_html.contains("<h1"));
}

#[test]
fn rejects_missing_front_matter() {
    assert!(Post::parse("s", "# h\n").is_err());
    assert!(Post::parse("s", "---\ntitle: t\n").is_err());
}

#[test]
fn draft_is_not_published() {
    let src = SRC.replace("tags: [a]", "isDraft: true");
    assert!(!Post::parse("s", &src).unwrap().meta.is_published());
}

fn post(slug: &str, date: &str, is_publish: bool, is_draft: bool, tags: &[&str]) -> Post {
    Post {
        slug: slug.into(),
        body_html: String::new(),
        meta: FrontMatter {
            title: slug.into(),
            published_at: date.into(),
            description: String::new(),
            is_publish,
            is_draft,
            tags: tags.iter().map(|tag| tag.to_string()).collect(),
            slug: None,
        },
    }
}

#[test]
fn published_filters_and_sorts() {
    let posts = vec![
        post("old", "2022-01-01", true, false, &[]),
        post("new", "2024-05-01", true, false, &[]),
        post("hidden", "2023-01-01", false, false, &[]),
        post("draft", "2023-02-01", true, true, &[]),
        post("future", "2999-01-01", true, false, &[]),
        post("same-day", "2024-05-01", true, false, &[]),
    ];
    let slugs: Vec<_> = published(posts, "2026-11-22")
        .into_iter()
        .map(|post| post.slug)
        .collect();
    assert_eq!(slugs, ["new", "same-day", "old"]);
}

#[test]
fn year_groups_keep_order() {
    let posts = published(
        vec![
            post("a", "2024-05-01", true, false, &[]),
            post("b", "2024-01-01", true, false, &[]),
            post("c", "2022-01-01", true, false, &[]),
        ],
        "2026-11-22",
    );
    let groups = group_by_year(&posts);
    assert_eq!(groups.len(), 2);
    assert_eq!(groups[0].0, "2024");
    assert_eq!(groups[0].1.len(), 2);
    assert_eq!(groups[1].0, "2022");
}

#[test]
fn tags_only_from_given_posts() {
    let posts = published(
        vec![
            post("a", "2024-01-01", true, false, &["x"]),
            post("b", "2024-01-02", false, false, &["secret"]),
        ],
        "2026-11-22",
    );
    let tags: Vec<_> = posts_by_tag(&posts).into_keys().collect();
    assert_eq!(tags, ["x"]);
}
