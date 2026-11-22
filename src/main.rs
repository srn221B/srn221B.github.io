use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use leptos::prelude::*;

use leptos_post::{
    pages::{
        home::HomePage, index::PostListPage, not_found::NotFoundPage, post_page::PostPage,
        tag::TagPage,
    },
    post::{self, Post},
    profile::LATEST_POSTS,
    site::BASE_PATH,
    util::today_iso,
};

/// view を HTML 文字列にする。Leptos の SSR は reactive owner の中で行う必要がある
fn render<V: IntoView>(view: impl FnOnce() -> V) -> String {
    let owner = Owner::new();
    owner.with(|| view().to_html())
}

fn write_page(path: PathBuf, html: String) -> Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let html = html.replace("<!>", "");
    fs::write(path, format!("<!DOCTYPE html>{html}"))?;
    Ok(())
}

fn remove_if_exists(path: &Path) -> Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let out_root = PathBuf::from(
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| "dist".to_string()),
    );
    let out_dir = out_root.join(BASE_PATH.trim_start_matches('/'));
    let content_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content/post");
    let today = today_iso()?;

    let all = Post::load_all(&content_dir)?;
    let total = all.len();

    for post in &all {
        remove_if_exists(&out_dir.join(&post.slug))?;
    }
    remove_if_exists(&out_dir.join("tags"))?;
    remove_if_exists(&out_dir.join("index.html"))?;
    remove_if_exists(&out_root.join("index.html"))?;
    remove_if_exists(&out_root.join("404.html"))?;

    let posts = post::published(all, &today);

    for post in &posts {
        write_page(
            out_dir.join(&post.slug).join("index.html"),
            render(|| view! { <PostPage post=post.clone() /> }),
        )?;
    }

    let tag_map = post::posts_by_tag(&posts);
    let tag_counts: Vec<(String, usize)> = tag_map
        .iter()
        .map(|(tag, tagged)| (tag.clone(), tagged.len()))
        .collect();

    let latest = posts
        .iter()
        .take(LATEST_POSTS)
        .map(Post::summary)
        .collect::<Vec<_>>();
    let home_tags = tag_counts.clone();
    write_page(
        out_root.join("index.html"),
        render(|| view! { <HomePage latest=latest tags=home_tags /> }),
    )?;

    let groups = post::group_by_year(&posts);
    write_page(
        out_dir.join("index.html"),
        render(|| view! { <PostListPage groups=groups tags=tag_counts /> }),
    )?;

    write_page(
        out_root.join("404.html"),
        render(|| view! { <NotFoundPage /> }),
    )?;

    for (tag, tagged) in &tag_map {
        let groups = post::group_by_year(tagged);
        write_page(
            out_dir.join("tags").join(tag).join("index.html"),
            render(|| view! { <TagPage tag=tag.clone() groups=groups /> }),
        )?;
    }

    println!(
        "generated: home, {} posts, {} tags, 1 index -> {} (skipped {} unpublished; today = {today})",
        posts.len(),
        tag_map.len(),
        out_root.display(),
        total - posts.len(),
    );
    Ok(())
}
