use leptos::prelude::*;

use crate::{
    components::{layout::Layout, post_card::TagList},
    post::Post,
    site::{absolute_url, index_href, post_href},
    util::urlencode,
};

#[component]
pub fn PostPage(post: Post) -> impl IntoView {
    let Post {
        slug,
        meta,
        body_html,
    } = post;
    let path = post_href(&slug);
    let url = absolute_url(&path);
    let share_href = format!(
        "https://x.com/intent/tweet?text={}&url={}",
        urlencode(&format!("{} @srn467o", meta.title)),
        urlencode(&url),
    );
    let edit_href = format!("https://github.com/srn221B/blog/blob/master/post/{slug}.md");

    view! {
        <Layout
            title=meta.title.clone()
            description=meta.description.clone()
            path=path
            og_type="article"
        >
            <main class="mx-auto flex w-full max-w-3xl flex-col gap-6 px-4 py-6">
                <div class="breadcrumbs text-sm">
                    <ul>
                        <li><a href="/">"Home"</a></li>
                        <li><a href=index_href()>"Posts"</a></li>
                        <li>{meta.title.clone()}</li>
                    </ul>
                </div>

                <header class="card bg-base-200 shadow">
                    <div class="card-body gap-3">
                        <h1 class="card-title text-2xl">{meta.title.clone()}</h1>
                        <p class="text-sm opacity-70">{meta.published_at.clone()}</p>
                        {(!meta.description.is_empty())
                            .then(|| view! { <p>{meta.description.clone()}</p> })}
                        <TagList tags=meta.tags.clone() />
                    </div>
                </header>

                <article class="prose prose-invert max-w-none" inner_html=body_html></article>

                <div class="divider"></div>

                <footer class="flex flex-col items-center gap-3 text-sm">
                    <a
                        href=share_href
                        target="_blank"
                        rel="noopener"
                        class="btn btn-sm btn-outline"
                    >
                        "Share on X"
                    </a>
                    <p class="opacity-70">
                        "Found a typo or something wrong? Please "
                        <a href=edit_href target="_blank" class="link link-info">"suggest a change"</a>
                    </p>
                </footer>
            </main>
        </Layout>
    }
}
