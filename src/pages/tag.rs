use leptos::prelude::*;

use crate::{
    components::{layout::Layout, post_card::YearGroups},
    post::Summary,
    site::tag_href,
};

#[component]
pub fn TagPage(tag: String, groups: Vec<(String, Vec<Summary>)>) -> impl IntoView {
    let title = format!("Posts tagged with \"{tag}\"");
    let path = tag_href(&tag);

    view! {
        <Layout title=title.clone() description="" path=path og_type="website">
            <main class="mx-auto flex w-full max-w-3xl flex-col gap-10 px-4 py-8">
                <h1 class="text-2xl font-bold">{title}</h1>
                <YearGroups groups=groups />
            </main>
        </Layout>
    }
}
