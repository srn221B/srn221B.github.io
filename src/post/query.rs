use std::collections::BTreeMap;

use super::{Post, Summary};

pub fn published(posts: Vec<Post>, today: &str) -> Vec<Post> {
    let mut posts: Vec<Post> = posts
        .into_iter()
        .filter(|post| post.is_published_on(today))
        .collect();
    posts.sort_by(|a, b| {
        b.meta
            .published_at
            .cmp(&a.meta.published_at)
            .then_with(|| a.slug.cmp(&b.slug))
    });
    posts
}

/// 新しい順に並んだ記事を年ごとにまとめる（年も新しい順のまま）
pub fn group_by_year(posts: &[Post]) -> Vec<(String, Vec<Summary>)> {
    let mut groups: Vec<(String, Vec<Summary>)> = Vec::new();
    for post in posts {
        let year = post.meta.published_at[..4].to_string();
        match groups.last_mut() {
            Some((last, list)) if *last == year => list.push(post.summary()),
            _ => groups.push((year, vec![post.summary()])),
        }
    }
    groups
}

pub fn posts_by_tag(posts: &[Post]) -> BTreeMap<String, Vec<Post>> {
    let mut map: BTreeMap<String, Vec<Post>> = BTreeMap::new();
    for post in posts {
        for tag in &post.meta.tags {
            map.entry(tag.clone()).or_default().push(post.clone());
        }
    }
    map
}
