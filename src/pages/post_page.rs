use leptos::prelude::*;

use crate::{
    components::{giscus::Giscus, layout::Layout, post_card::TagList, toc::Toc},
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
        toc,
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
            <main class="relative mx-auto flex w-full max-w-3xl flex-col gap-6 px-4 py-6">
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

                {(!toc.is_empty())
                    .then(|| {
                        view! {
                            <details class="rounded-box bg-base-200 p-4 xl:hidden">
                                <summary class="cursor-pointer text-sm font-bold">"目次"</summary>
                                <div class="mt-3">
                                    <Toc entries=toc.clone() />
                                </div>
                            </details>
                            <aside class="absolute left-full top-6 bottom-6 ml-8 hidden w-56 xl:block">
                                <div class="sticky top-6 max-h-[calc(100vh-3rem)] overflow-y-auto">
                                    <p class="mb-3 text-sm font-bold">"目次"</p>
                                    <Toc entries=toc />
                                </div>
                            </aside>
                        }
                    })}

                <article class="prose prose-invert max-w-none" inner_html=body_html></article>

                <div class="divider"></div>

                <Giscus />

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
