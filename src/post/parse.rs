use std::{fs, path::Path};

use anyhow::{Context, Result, bail};

use super::{FrontMatter, Post};
use crate::{
    markdown::{Rendered, render_markdown},
    util::is_iso_date,
};

impl Post {
    pub fn load(dir: &Path, slug: &str) -> Result<Self> {
        Self::load_file(&dir.join(format!("{slug}.md")))
    }

    pub fn load_all(dir: &Path) -> Result<Vec<Self>> {
        let mut paths: Vec<_> = fs::read_dir(dir)
            .with_context(|| format!("{} を開けません", dir.display()))?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
            .collect();
        paths.sort();
        paths.iter().map(|path| Self::load_file(path)).collect()
    }

    pub fn parse(slug: &str, src: &str) -> Result<Self> {
        let (yaml, body) = split_front_matter(src)?;
        let meta: FrontMatter = serde_yaml_ng::from_str(yaml).context("frontmatter が不正です")?;
        validate(slug, &meta)?;

        let Rendered { html, toc } = render_markdown(body);
        Ok(Self {
            slug: slug.to_string(),
            meta,
            body_html: html,
            toc,
        })
    }

    fn load_file(path: &Path) -> Result<Self> {
        let slug = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .with_context(|| format!("{} のファイル名が不正です", path.display()))?
            .to_string();
        let src =
            fs::read_to_string(path).with_context(|| format!("{} を読めません", path.display()))?;
        Self::parse(&slug, &src)
            .with_context(|| format!("{} の読み込みに失敗しました", path.display()))
    }
}

fn split_front_matter(src: &str) -> Result<(&str, &str)> {
    let rest = src
        .strip_prefix("---")
        .context("frontmatter (---) が見つかりません")?;
    let end = rest
        .find("\n---")
        .context("frontmatter の終わりが見つかりません")?;
    Ok((&rest[..end], &rest[end + 4..]))
}

fn validate(slug: &str, meta: &FrontMatter) -> Result<()> {
    if !is_iso_date(&meta.published_at) {
        bail!(
            "publishedAt は YYYY-MM-DD で書いてください: {}",
            meta.published_at
        );
    }
    if let Some(declared) = &meta.slug
        && declared != slug
    {
        bail!(
            "frontmatter の slug ({declared}) がファイル名 ({slug}) と違います。URL が変わるため確認してください"
        );
    }
    for tag in &meta.tags {
        if tag.is_empty() || tag.contains(['/', '\\']) || tag.starts_with('.') {
            bail!("使えないタグ名です: {tag:?}");
        }
    }
    Ok(())
}
