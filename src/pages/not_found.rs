use leptos::prelude::*;

use crate::{components::layout::Layout, site::index_href};

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Layout title="Not Found" description="" path="/404.html" og_type="website">
            <main class="mx-auto flex w-full max-w-3xl flex-col items-center gap-6 px-4 py-24 text-center">
                <p class="text-7xl font-bold text-sky-400 md:text-9xl">"404"</p>
                <h1 class="text-xl font-bold">"Not Found"</h1>
                <p class="opacity-70">"お探しのページは見つかりませんでした。"</p>
                <div class="flex gap-3">
                    <a href="/" class="btn btn-sm btn-ghost border border-sky-400">"Home"</a>
                    <a href=index_href() class="btn btn-sm btn-ghost border border-sky-400">"Posts"</a>
                </div>
            </main>
        </Layout>
    }
}
