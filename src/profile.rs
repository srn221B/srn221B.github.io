use crate::components::icon::Icon;

pub const NAME: &str = "467 shirona";

pub const BIO: &[&[(&str, bool)]] = &[
    &[
        ("東京でエンジニアをやっています。", false),
        ("ここは個人ページ", true),
        ("で、雑多なブログを書いたり書かなかったり。", false),
    ],
    &[("出身：静岡(22年)→東京(4年)→静岡(1.5年)->東京", false)],
    &[("趣味：海外旅行、アニメ、など", false)],
];

pub const AVATAR_URL: &str = "https://user-images.githubusercontent.com/60976262/190943162-bd237020-2d79-4566-a426-bc6d048e1b8e.png";
pub const AVATAR_ALT: &str = "rabibibibibit";

pub struct Social {
    pub label: &'static str,
    pub icon: Icon,
    pub link: &'static str,
}

pub const SOCIALS: &[Social] = &[
    Social {
        label: "X (Twitter)",
        icon: Icon::Twitter,
        link: "https://x.com/srn467o",
    },
    Social {
        label: "LinkedIn",
        icon: Icon::LinkedIn,
        link: "https://www.linkedin.com/in/srn221b/",
    },
    Social {
        label: "GitHub",
        icon: Icon::GitHub,
        link: "https://github.com/srn221B",
    },
];

pub struct Hobby {
    pub title: &'static str,
    pub description: &'static str,
    pub link: &'static str,
}

pub const HOBBIES: &[Hobby] = &[
    Hobby {
        title: "Zenn",
        description: "技術ブログ",
        link: "https://zenn.dev/467",
    },
    Hobby {
        title: "Filmarks",
        description: "アニメとか映画とか見ているものの記録",
        link: "https://filmarks.com/users/467",
    },
    Hobby {
        title: "Hatena",
        description: "海外旅行の記録",
        link: "https://s6n.hatenablog.com/",
    },
    Hobby {
        title: "Note",
        description: "完全に趣味用のブログ",
        link: "https://note.com/srn221b",
    },
];

pub const LATEST_POSTS: usize = 2;
