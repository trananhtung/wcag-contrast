# wcag-contrast

[![Crates.io](https://img.shields.io/crates/v/wcag-contrast.svg)](https://crates.io/crates/wcag-contrast)
[![Documentation](https://docs.rs/wcag-contrast/badge.svg)](https://docs.rs/wcag-contrast)
[![CI](https://github.com/trananhtung/wcag-contrast/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/wcag-contrast/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/wcag-contrast.svg)](#license)

**Color contrast for accessibility.** Compute [WCAG 2](https://www.w3.org/TR/WCAG21/#contrast-minimum)
contrast ratios from hex or RGB colors and check whether they pass **AA / AAA**
for normal or large text. A small, focused, **zero-dependency** helper.

```rust
use wcag_contrast::{contrast_hex, level, WcagLevel};

let ratio = contrast_hex("#777777", "#ffffff").unwrap();
assert!((ratio - 4.48).abs() < 0.01);          // ~4.48 : 1
assert_eq!(level(ratio, false), WcagLevel::Fail); // just under 4.5 → fails AA for normal text
assert_eq!(level(ratio, true), WcagLevel::AA);    // passes AA for large text (≥3.0)
```

## Why wcag-contrast?

Accessibility checks (WCAG AA/AAA contrast) show up in linters, design systems,
CI gates, and CSS tooling. The color math exists inside large general-purpose
color crates, but there was no small, dependency-free crate that just answers
*"what's the contrast ratio, and does it pass?"*. This is that crate.

```toml
[dependencies]
wcag-contrast = "0.1"
```

## API

| Item | Purpose |
| --- | --- |
| `Rgb::from_hex(&str)` | Parse `"#rgb"` / `"#rrggbb"` (with/without `#`, case-insensitive) |
| `Rgb::relative_luminance()` | WCAG relative luminance (0.0–1.0) |
| `contrast_ratio(a, b)` | Contrast ratio (1.0–21.0), order-independent |
| `contrast_hex(a, b)` | Same, straight from two hex strings |
| `level(ratio, large_text)` | Highest `WcagLevel` passed (`Fail` / `AA` / `AAA`) |

### Thresholds

| | AA | AAA |
| --- | --- | --- |
| Normal text | 4.5 : 1 | 7 : 1 |
| Large text (≥18pt, or ≥14pt bold) | 3 : 1 | 4.5 : 1 |

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.
