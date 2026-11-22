use crate::util::urlencode;

pub const BASE_PATH: &str = "/post";
pub const SITE_URL: &str = "https://srn221b.github.io";
pub const STYLE_HREF: &str = "/style.css";

pub fn index_href() -> String {
    format!("{BASE_PATH}/")
}

pub fn post_href(slug: &str) -> String {
    format!("{BASE_PATH}/{slug}/")
}

pub fn tag_href(tag: &str) -> String {
    format!("{BASE_PATH}/tags/{}/", urlencode(tag))
}

pub fn absolute_url(path: &str) -> String {
    format!("{SITE_URL}{path}")
}
