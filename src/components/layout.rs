use leptos::prelude::*;

use crate::{
    components::{blur_circle::BlurCircles, head::og},
    site::{STYLE_HREF, absolute_url, index_href},
};

#[component]
pub fn Layout(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    /// サイト内のパス（例: `/post/content2/`）。canonical と og:url に使う
    #[prop(into)]
    path: String,
    og_type: &'static str,
    children: Children,
) -> impl IntoView {
    let url = absolute_url(&path);
    let page_title = format!("{title} | srn221b");
    let meta_description = (!description.is_empty())
        .then(|| view! { <meta name="description" content=description.clone() /> });

    view! {
        <html lang="ja" data-theme="dark">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>{page_title}</title>
                {meta_description}
                {og("og:type", og_type.to_string())}
                {og("og:title", title)}
                {og("og:description", description)}
                {og("og:url", url.clone())}
                <meta name="twitter:card" content="summary" />
                <link rel="canonical" href=url />
                <link rel="icon" href="/favicon.svg" />
                <link rel="stylesheet" href=STYLE_HREF />
            </head>
            <body class="min-h-screen bg-base-100 text-base-content">
                <BlurCircles />
                <header class="navbar relative z-10 mx-auto max-w-3xl px-4">
                    <a href="/" aria-label="srn221b" class="btn btn-ghost btn-square">
                        <img src="/favicon.svg" alt="" width="28" height="28" />
                    </a>
                    <nav class="ml-auto">
                        <a href=index_href() class="btn btn-ghost btn-sm">"Posts"</a>
                    </nav>
                </header>
                <div class="relative z-10">{children()}</div>
                <footer class="relative z-10 mx-auto max-w-3xl px-4 py-10 text-center text-sm opacity-70">
                    <a class="link" href="https://github.com/srn221B/srn221B.github.io">
                        "©2026 srn221b"
                    </a>
                </footer>
            </body>
        </html>
    }
}
