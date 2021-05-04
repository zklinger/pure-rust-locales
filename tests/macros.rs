use std::convert::TryInto;

#[test]
fn locale_match() {
    let locale = "fr_BE".try_into().unwrap();
    let result = pure_rust_locales::locale_match!(locale => LC_TIME::D_FMT);
    assert_eq!(result, "%d/%m/%y");
}

#[test]
fn locale_match_value_exists() {
    let locale = "hu_HU".try_into().unwrap();
    let result = pure_rust_locales::locale_match!(locale => LC_TIME::FIRST_WEEKDAY);
    assert_eq!(result, 2);
}

#[test]
fn locale_match_value_generated() {
    let locale = "en_AU".try_into().unwrap();
    let result = pure_rust_locales::locale_match!(locale => LC_TIME::FIRST_WEEKDAY);
    assert_eq!(result, 1);
}

