use leptos::prelude::*;

use crate::markdown::TocEntry;

#[component]
pub fn Toc(entries: Vec<TocEntry>) -> impl IntoView {
    let top_level = entries.iter().map(|entry| entry.level).min()?;

    Some(view! {
        <nav aria-label="目次">
            <ul class="flex flex-col gap-1.5 border-l border-base-content/20 text-sm">
                {entries
                    .into_iter()
                    .map(|entry| {
                        let indent = match entry.level - top_level {
                            0 => "pl-3",
                            1 => "pl-6",
                            _ => "pl-9",
                        };
                        view! {
                            <li class=indent>
                                <a href=format!("#{}", entry.id) class="opacity-70 hover:text-sky-400 hover:opacity-100">
                                    {entry.text}
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </nav>
    })
}
