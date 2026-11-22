use crate::util::urlencode;

pub const BASE_PATH: &str = "/post";
pub const SITE_URL: &str = "https://srn221b.github.io";
pub const OG_IMAGE: &str = "https://user-images.githubusercontent.com/60976262/190943162-bd237020-2d79-4566-a426-bc6d048e1b8e.png";
pub const OG_IMAGE_ALT: &str = "rabibibibibibit";
pub const TWITTER_CREATOR: &str = "@srn467o";
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
