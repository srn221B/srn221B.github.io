use leptos::prelude::*;

use crate::{
    post::Summary,
    site::{post_href, tag_href},
};

#[component]
pub fn TagList(tags: Vec<String>) -> impl IntoView {
    (!tags.is_empty()).then(|| {
        view! {
            <div class="flex flex-wrap gap-2">
                {tags
                    .into_iter()
                    .map(|tag| {
                        let href = tag_href(&tag);
                        view! { <a href=href class="badge badge-dash badge-secondary badge-sm">{tag}</a> }
                    })
                    .collect_view()}
            </div>
        }
    })
}

#[component]
pub fn TagCounts(tags: Vec<(String, usize)>) -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2">
            {tags
                .into_iter()
                .map(|(tag, count)| {
                    let href = tag_href(&tag);
                    view! {
                        <a href=href class="badge badge-dash badge-secondary badge-sm">
                            {format!("{tag} ({count})")}
                        </a>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn PostCard(post: Summary) -> impl IntoView {
    let Summary {
        slug,
        title,
        published_at,
        description,
        tags,
    } = post;
    let href = post_href(&slug);

    view! {
        <div class="card border border-sky-400 transition">
            <div class="card-body gap-2 p-4">
                <h3 class="card-title text-base">
                    <a href=href class="link link-hover">{title}</a>
                </h3>
                <p class="text-xs opacity-70">{published_at}</p>
                {(!description.is_empty()).then(|| view! { <p class="text-sm">{description.clone()}</p> })}
                <TagList tags=tags />
            </div>
        </div>
    }
}

#[component]
pub fn YearGroups(groups: Vec<(String, Vec<Summary>)>) -> impl IntoView {
    groups
        .into_iter()
        .map(|(year, posts)| {
            view! {
                <section class="flex flex-col gap-4">
                    <h2 class="text-xl font-bold opacity-80">{year}</h2>
                    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                        {posts
                            .into_iter()
                            .map(|post| view! { <PostCard post=post /> })
                            .collect_view()}
                    </div>
                </section>
            }
        })
        .collect_view()
}
