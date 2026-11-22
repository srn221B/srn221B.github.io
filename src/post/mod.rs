mod parse;
mod query;

use serde::Deserialize;

use crate::markdown::TocEntry;

pub use query::{group_by_year, posts_by_tag, published};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontMatter {
    pub title: String,
    /// `YYYY-MM-DD`
    pub published_at: String,
    #[serde(default)]
    pub description: String,
    pub is_publish: bool,
    #[serde(default)]
    pub is_draft: bool,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub slug: Option<String>,
}

impl FrontMatter {
    pub fn is_published(&self) -> bool {
        self.is_publish && !self.is_draft
    }

    pub fn is_published_on(&self, today: &str) -> bool {
        self.is_published() && self.published_at.as_str() <= today
    }
}

#[derive(Debug, Clone)]
pub struct Post {
    pub slug: String,
    pub meta: FrontMatter,
    pub body_html: String,
    pub toc: Vec<TocEntry>,
}

#[derive(Debug, Clone)]
pub struct Summary {
    pub slug: String,
    pub title: String,
    pub published_at: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl Post {
    pub fn is_published_on(&self, today: &str) -> bool {
        self.meta.is_published_on(today)
    }

    pub fn summary(&self) -> Summary {
        Summary {
            slug: self.slug.clone(),
            title: self.meta.title.clone(),
            published_at: self.meta.published_at.clone(),
            description: self.meta.description.clone(),
            tags: self.meta.tags.clone(),
        }
    }
}
