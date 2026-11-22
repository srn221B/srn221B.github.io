use leptos::prelude::*;

/// property は Leptos の型付き属性に無いため .attr で付ける
pub fn og(property: &'static str, content: String) -> impl IntoView {
    leptos::html::meta()
        .attr("property", property)
        .attr("content", content)
}
