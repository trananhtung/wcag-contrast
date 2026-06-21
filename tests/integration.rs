//! End-to-end behavioral spec for the public `wcag-contrast` API.

use wcag_contrast::{contrast_hex, contrast_ratio, level, ParseError, Rgb, WcagLevel};

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.01
}

// ---------------------------------------------------------------------------
// hex parsing
// ---------------------------------------------------------------------------

#[test]
fn parse_six_and_three_digit_hex() {
    assert_eq!(Rgb::from_hex("#ffffff").unwrap(), Rgb::new(255, 255, 255));
    assert_eq!(Rgb::from_hex("ffffff").unwrap(), Rgb::new(255, 255, 255));
    assert_eq!(Rgb::from_hex("#000").unwrap(), Rgb::new(0, 0, 0));
    assert_eq!(Rgb::from_hex("abc").unwrap(), Rgb::new(0xaa, 0xbb, 0xcc));
    assert_eq!(Rgb::from_hex("#FFF").unwrap(), Rgb::new(255, 255, 255)); // case-insensitive
    assert_eq!(
        Rgb::from_hex("#aAbBcC").unwrap(),
        Rgb::new(0xaa, 0xbb, 0xcc)
    );
}

#[test]
fn parse_hex_errors() {
    assert!(Rgb::from_hex("").is_err());
    assert!(Rgb::from_hex("#ff").is_err()); // wrong length
    assert!(Rgb::from_hex("12345").is_err());
    assert!(Rgb::from_hex("#ggg").is_err()); // not hex
    assert!(matches!(
        Rgb::from_hex("xyz"),
        Err(ParseError::InvalidHex(_))
    ));
}

#[test]
fn from_hex_does_not_panic_on_multibyte() {
    // Multibyte UTF-8 must return Err, never panic on a non-char-boundary slice.
    for s in ["€", "￥", "€€", "ab🎨", "café", "é", "#é"] {
        assert!(
            Rgb::from_hex(s).is_err(),
            "{s:?} should be Err, not panic/Ok"
        );
    }
}

// ---------------------------------------------------------------------------
// luminance & contrast
// ---------------------------------------------------------------------------

#[test]
fn relative_luminance_extremes() {
    assert!(close(Rgb::new(255, 255, 255).relative_luminance(), 1.0));
    assert!(close(Rgb::new(0, 0, 0).relative_luminance(), 0.0));
}

#[test]
fn contrast_ratio_extremes() {
    let white = Rgb::new(255, 255, 255);
    let black = Rgb::new(0, 0, 0);
    assert!(close(contrast_ratio(white, black), 21.0));
    assert!(close(contrast_ratio(black, white), 21.0)); // order-independent
    assert!(close(contrast_ratio(white, white), 1.0));
}

#[test]
fn contrast_hex_known_values() {
    assert!(close(contrast_hex("#ffffff", "#000000").unwrap(), 21.0));
    // mid-gray #777 on white is the classic "just barely AA" pair (~4.48)
    let r = contrast_hex("#777777", "#ffffff").unwrap();
    assert!((4.4..4.6).contains(&r), "got {r}");
}

#[test]
fn contrast_hex_propagates_parse_errors() {
    assert!(contrast_hex("nope", "#000").is_err());
}

// ---------------------------------------------------------------------------
// WCAG levels
// ---------------------------------------------------------------------------

#[test]
fn levels_for_normal_text() {
    assert_eq!(level(21.0, false), WcagLevel::AAA);
    assert_eq!(level(7.0, false), WcagLevel::AAA);
    assert_eq!(level(4.5, false), WcagLevel::AA);
    assert_eq!(level(4.49, false), WcagLevel::Fail);
}

#[test]
fn levels_for_large_text() {
    assert_eq!(level(4.5, true), WcagLevel::AAA);
    assert_eq!(level(3.0, true), WcagLevel::AA);
    assert_eq!(level(2.99, true), WcagLevel::Fail);
}

#[test]
fn level_display() {
    assert_eq!(WcagLevel::AAA.to_string(), "AAA");
    assert_eq!(WcagLevel::AA.to_string(), "AA");
    assert_eq!(WcagLevel::Fail.to_string(), "Fail");
}
