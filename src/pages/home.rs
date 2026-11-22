use leptos::prelude::*;

use crate::{
    components::{
        icon::SvgIcon,
        layout::Layout,
        post_card::{PostCard, TagCounts},
    },
    post::Summary,
    profile::{AVATAR_ALT, AVATAR_URL, BIO, HOBBIES, NAME, SOCIALS},
    site::index_href,
};

#[component]
pub fn HomePage(latest: Vec<Summary>, tags: Vec<(String, usize)>) -> impl IntoView {
    view! {
        <Layout title=NAME description="" path="/" og_type="website">
            <main class="mx-auto flex w-full max-w-3xl flex-col gap-16 px-4 py-8">
                <section class="flex flex-col-reverse items-center gap-8 md:flex-row md:items-start">
                    <div class="flex flex-1 flex-col gap-6">
                        <h1 class="text-3xl font-bold">{NAME}</h1>
                        <div class="flex flex-col gap-4 leading-7">
                            {BIO
                                .iter()
                                .map(|paragraph| {
                                    view! {
                                        <p>
                                            {paragraph
                                                .iter()
                                                .map(|&(text, strong)| {
                                                    if strong {
                                                        view! { <strong class="font-normal text-secondary">{text}</strong> }
                                                            .into_any()
                                                    } else {
                                                        text.into_any()
                                                    }
                                                })
                                                .collect_view()}
                                        </p>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>
                    <img src=AVATAR_URL alt=AVATAR_ALT width="150" height="150" class="shrink-0" />
                </section>

                <section class="flex flex-col gap-4">
                    <h2 class="text-lg font-bold">"Social Links"</h2>
                    <ul class="flex flex-row gap-3">
                        {SOCIALS
                            .iter()
                            .map(|social| {
                                view! {
                                    <li>
                                        <a
                                            href=social.link
                                            target="_blank"
                                            rel="noopener"
                                            aria-label=social.label
                                            title=social.label
                                            class="btn btn-square btn-ghost border border-sky-400"
                                        >
                                            <SvgIcon icon=social.icon />
                                        </a>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                </section>

                <section class="flex flex-col gap-4">
                    <div class="flex items-center justify-between">
                        <h2 class="text-lg font-bold">"Latest posts"</h2>
                        <a href=index_href() class="link link-hover text-sm">"See all posts →"</a>
                    </div>
                    {if latest.is_empty() {
                        view! { <p>"Soon, stay connected 👀..."</p> }.into_any()
                    } else {
                        view! {
                            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                                {latest.into_iter().map(|post| view! { <PostCard post=post /> }).collect_view()}
                            </div>
                        }
                            .into_any()
                    }}
                    <TagCounts tags=tags />
                </section>

                <section class="flex flex-col gap-4">
                    <h2 class="text-lg font-bold">"Hobbies links"</h2>
                    <ul class="flex flex-col gap-2">
                        {HOBBIES
                            .iter()
                            .map(|hobby| {
                                view! {
                                    <li>
                                        <a
                                            href=hobby.link
                                            target="_blank"
                                            rel="noopener"
                                            class="group flex items-center justify-between gap-4 rounded-box border border-sky-400/60 bg-base-200 px-4 py-3 transition hover:border-sky-400 hover:bg-base-300"
                                        >
                                            <span class="flex flex-col">
                                                <span class="font-semibold">{hobby.title}</span>
                                                <span class="text-xs opacity-70">{hobby.description}</span>
                                            </span>
                                            <span class="text-sky-400 transition group-hover:translate-x-1">"↗"</span>
                                        </a>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                </section>
            </main>
        </Layout>
    }
}
