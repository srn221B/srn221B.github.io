use leptos_post::util::{civil_from_days, is_iso_date, today_or, urlencode};

#[test]
fn urlencode_japanese() {
    assert_eq!(
        urlencode("キーボード"),
        "%E3%82%AD%E3%83%BC%E3%83%9C%E3%83%BC%E3%83%89"
    );
    assert_eq!(urlencode("Python"), "Python");
}

#[test]
fn civil_from_days_works() {
    assert_eq!(civil_from_days(0), (1970, 1, 1));
    assert_eq!(civil_from_days(19_723), (2024, 1, 1));
    assert_eq!(civil_from_days(20_508), (2026, 2, 24));
}

#[test]
fn iso_date_check() {
    assert!(is_iso_date("2024-05-01"));
    assert!(!is_iso_date("2024-5-1"));
    assert!(!is_iso_date("20240501"));
}

#[test]
fn today_or_uses_valid_override() {
    assert_eq!(today_or(Some("2026-01-02")).unwrap(), "2026-01-02");
}

#[test]
fn today_or_rejects_invalid_override() {
    assert!(today_or(Some("abc")).is_err());
    assert!(today_or(Some("2026-1-2")).is_err());
}

#[test]
fn today_or_defaults_to_iso_date() {
    assert!(is_iso_date(&today_or(None).unwrap()));
}
