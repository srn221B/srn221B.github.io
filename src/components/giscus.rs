use leptos::prelude::*;

#[component]
pub fn Giscus() -> impl IntoView {
    view! {
        <section class="w-full">
            <script
                src="https://giscus.app/client.js"
                data-repo="srn221B/blog"
                data-repo-id="R_kgDOIBf-Xg"
                data-category="Comments"
                data-category-id="DIC_kwDOIBf-Xs4CTe7o"
                data-mapping="pathname"
                data-strict="0"
                data-reactions-enabled="1"
                data-emit-metadata="0"
                data-input-position="bottom"
                data-theme="dark"
                crossorigin="anonymous"
                async
            ></script>
        </section>
    }
}
