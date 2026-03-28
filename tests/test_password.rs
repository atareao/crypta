use crypta::secrets;

#[test]
fn password_no_special_has_correct_length_and_charset() {
    let pw = secrets::password_string(16, false).expect("failed to generate password");
    assert_eq!(pw.len(), 16);
    assert!(pw.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[test]
fn password_with_special_includes_special_char() {
    let pw = secrets::password_string(24, true).expect("failed to generate password");
    assert_eq!(pw.len(), 24);
    let specials = "!@#$%^&*()-_=+[]{};:,.<>?/|\\";
    assert!(pw.chars().any(|c| specials.contains(c)));
}
