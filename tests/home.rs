use leptos::prelude::*;
use leptos_post::{
    pages::home::HomePage,
    post::Summary,
    profile::{HOBBIES, SOCIALS},
};

fn summary(slug: &str) -> Summary {
    Summary {
        slug: slug.into(),
        title: format!("title-{slug}"),
        published_at: "2025-01-01".into(),
        description: String::new(),
        tags: vec!["Python".into()],
    }
}

fn render_home(latest: Vec<Summary>, tags: Vec<(String, usize)>) -> String {
    let owner = Owner::new();
    owner.with(|| view! { <HomePage latest=latest tags=tags /> }.to_html())
}

#[test]
fn shows_latest_posts_tags_and_links() {
    let html = render_home(vec![summary("a"), summary("b")], vec![("Python".into(), 5)]);
    assert!(html.contains(r#"href="/post/a/""#));
    assert!(html.contains(r#"href="/post/b/""#));
    assert!(html.contains("Python (5)"));
    assert!(
        html.contains(r#"href="/post/""#),
        "See all posts へのリンク"
    );
    for social in SOCIALS {
        assert!(html.contains(social.link));
    }
    for hobby in HOBBIES {
        assert!(html.contains(hobby.link));
    }
    assert!(html.contains(r#"href="/style.css""#));
}

#[test]
fn shows_placeholder_without_posts() {
    let html = render_home(vec![], vec![]);
    assert!(html.contains("Soon, stay connected"));
}
