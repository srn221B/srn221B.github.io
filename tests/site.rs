use leptos_post::site::{absolute_url, post_href, tag_href};

#[test]
fn hrefs() {
    assert_eq!(post_href("content2"), "/post/content2/");
    assert_eq!(tag_href("Python"), "/post/tags/Python/");
    assert_eq!(
        tag_href("キーボード"),
        "/post/tags/%E3%82%AD%E3%83%BC%E3%83%9C%E3%83%BC%E3%83%89/"
    );
    assert_eq!(
        absolute_url(&post_href("content2")),
        "https://srn221b.github.io/post/content2/"
    );
}
