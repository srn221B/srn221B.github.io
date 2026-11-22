use leptos::prelude::*;

use crate::{
    components::{
        layout::Layout,
        post_card::{TagCounts, YearGroups},
    },
    post::Summary,
    site::index_href,
};

#[component]
pub fn PostListPage(
    groups: Vec<(String, Vec<Summary>)>,
    tags: Vec<(String, usize)>,
) -> impl IntoView {
    view! {
        <Layout title="Posts" description="" path=index_href() og_type="website">
            <main class="mx-auto flex w-full max-w-3xl flex-col gap-10 px-4 py-8">
                <h1 class="text-2xl font-bold">"Posts"</h1>
                <TagCounts tags=tags />
                <YearGroups groups=groups />
            </main>
        </Layout>
    }
}
