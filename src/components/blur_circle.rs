use leptos::prelude::*;

#[component]
pub fn BlurCircles() -> impl IntoView {
    view! {
        <div class="pointer-events-none fixed left-0 top-0 z-0 h-[134px] w-[134px] rounded-full bg-pink-400 opacity-50 blur-[150px] md:blur-[350px] lg:h-[300px] lg:w-[300px]"></div>
        <div class="pointer-events-none fixed bottom-0 right-0 z-0 h-[134px] w-[134px] rounded-full bg-violet-400 opacity-50 blur-[150px] md:blur-[350px] lg:h-[300px] lg:w-[300px]"></div>
    }
}
