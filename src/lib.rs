//! # wcag-contrast — color contrast for accessibility
//!
//! Compute [WCAG 2](https://www.w3.org/TR/WCAG21/#contrast-minimum) color
//! contrast ratios from hex or RGB colors and check whether they pass AA / AAA
//! for normal or large text. A small, focused, zero-dependency accessibility
//! helper for linters, design tools, and CI checks.
//!
//! ```
//! use wcag_contrast::{contrast_hex, level, WcagLevel};
//!
//! let ratio = contrast_hex("#ffffff", "#000000").unwrap();
//! assert_eq!(ratio, 21.0); // maximum possible contrast
//! assert_eq!(level(ratio, false), WcagLevel::AAA);
//! ```

use std::fmt;

/// An error produced while parsing a hex color.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseError {
    /// The hex string was not 3 or 6 digits (ignoring an optional leading `#`).
    WrongLength(usize),
    /// The string contained non-hexadecimal characters.
    InvalidHex(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::WrongLength(n) => {
                write!(f, "hex color must have 3 or 6 digits, found {n}")
            }
            ParseError::InvalidHex(s) => write!(f, "invalid hex color: {s:?}"),
        }
    }
}

impl std::error::Error for ParseError {}

/// An 8-bit-per-channel sRGB color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb {
    /// Red channel (0–255).
    pub r: u8,
    /// Green channel (0–255).
    pub g: u8,
    /// Blue channel (0–255).
    pub b: u8,
}

impl Rgb {
    /// Create a color from its channels.
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Parse a hex color (`"#rgb"`, `"#rrggbb"`, with or without `#`,
    /// case-insensitive).
    ///
    /// # Errors
    ///
    /// Returns [`ParseError`] if the length is not 3 or 6, or the digits are not
    /// valid hexadecimal.
    pub fn from_hex(s: &str) -> Result<Self, ParseError> {
        let digits = s.strip_prefix('#').unwrap_or(s);
        // Hex is ASCII; rejecting non-ASCII up front keeps byte indexing safe
        // (so multibyte input returns Err instead of panicking) and makes the
        // length count characters.
        if !digits.is_ascii() {
            return Err(ParseError::InvalidHex(s.to_owned()));
        }
        let bytes = digits.as_bytes();
        let expanded: [u8; 6] = match bytes.len() {
            3 => [bytes[0], bytes[0], bytes[1], bytes[1], bytes[2], bytes[2]],
            6 => [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]],
            n => return Err(ParseError::WrongLength(n)),
        };
        let channel = |hi: u8, lo: u8| -> Result<u8, ParseError> {
            let pair = [hi, lo];
            let text =
                core::str::from_utf8(&pair).map_err(|_| ParseError::InvalidHex(s.to_owned()))?;
            u8::from_str_radix(text, 16).map_err(|_| ParseError::InvalidHex(s.to_owned()))
        };
        Ok(Self::new(
            channel(expanded[0], expanded[1])?,
            channel(expanded[2], expanded[3])?,
            channel(expanded[4], expanded[5])?,
        ))
    }

    /// The WCAG relative luminance of this color, in `0.0..=1.0`.
    #[must_use]
    pub fn relative_luminance(&self) -> f64 {
        fn linearize(channel: u8) -> f64 {
            let c = f64::from(channel) / 255.0;
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        0.2126 * linearize(self.r) + 0.7152 * linearize(self.g) + 0.0722 * linearize(self.b)
    }
}

/// The WCAG 2 contrast ratio between two colors, from `1.0` (identical) to
/// `21.0` (black vs white). Order-independent.
#[must_use]
pub fn contrast_ratio(a: Rgb, b: Rgb) -> f64 {
    let la = a.relative_luminance();
    let lb = b.relative_luminance();
    let (lighter, darker) = if la >= lb { (la, lb) } else { (lb, la) };
    (lighter + 0.05) / (darker + 0.05)
}

/// Parse two hex colors and return their contrast ratio.
///
/// # Errors
///
/// Returns [`ParseError`] if either color fails to parse.
pub fn contrast_hex(a: &str, b: &str) -> Result<f64, ParseError> {
    Ok(contrast_ratio(Rgb::from_hex(a)?, Rgb::from_hex(b)?))
}

/// The highest WCAG conformance level a contrast `ratio` meets for the given
/// text size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WcagLevel {
    /// Does not meet AA.
    Fail,
    /// Meets AA (4.5:1 normal, 3:1 large) but not AAA.
    AA,
    /// Meets AAA (7:1 normal, 4.5:1 large).
    AAA,
}

impl fmt::Display for WcagLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            WcagLevel::Fail => "Fail",
            WcagLevel::AA => "AA",
            WcagLevel::AAA => "AAA",
        })
    }
}

/// Classify a contrast `ratio` into the highest [`WcagLevel`] it passes for
/// normal or `large_text` (≥18pt, or ≥14pt bold).
#[must_use]
pub fn level(ratio: f64, large_text: bool) -> WcagLevel {
    let (aa, aaa) = if large_text { (3.0, 4.5) } else { (4.5, 7.0) };
    if ratio >= aaa {
        WcagLevel::AAA
    } else if ratio >= aa {
        WcagLevel::AA
    } else {
        WcagLevel::Fail
    }
}
