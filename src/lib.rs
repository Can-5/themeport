//! ThemePort: theme profiles and theme-pack manifests for Orbit.
//!
//! A ThemePort *pack* is pure data: validated hex colors, metadata, checksums. It can
//! never carry executable content — `PackManifest` uses `deny_unknown_fields` and its
//! validator rejects anything that smells like a script hook (unknown fields fail at
//! parse time; ids must be slugs; versions must be semver-ish). Applying a theme to the
//! OS shell belongs to the Orbit host.
//!
//! Contrast math follows WCAG 2.x relative luminance — computed manually so this crate
//! has zero graphics dependencies.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ThemePortError {
    #[error("expected #rrggbb hex color, got {0:?}")]
    BadColor(String),
    #[error("pack format must be 1, got {0}")]
    BadFormat(u32),
    #[error("id {0:?} is not a slug (lowercase alnum + '-')")]
    BadId(String),
    #[error("version {0:?} is not semver-ish (MAJOR.MINOR.PATCH)")]
    BadVersion(String),
    #[error("pack manifest lists no themes")]
    EmptyPack,
    #[error("duplicate theme id {0:?} in pack")]
    DuplicateThemeId(String),
}

/// sRGB hex color, validated and stored lowercase with leading '#'.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThemeColor(String);

impl ThemeColor {
    pub fn parse(input: &str) -> Result<Self, ThemePortError> {
        let s = input.trim();
        let hex = s.strip_prefix('#').unwrap_or(s);
        if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(Self(format!("#{}", hex.to_lowercase())))
        } else {
            Err(ThemePortError::BadColor(input.into()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn channels(&self) -> (u8, u8, u8) {
        let h = &self.0[1..];
        (
            u8::from_str_radix(&h[0..2], 16).unwrap(),
            u8::from_str_radix(&h[2..4], 16).unwrap(),
            u8::from_str_radix(&h[4..6], 16).unwrap(),
        )
    }
}

/// WCAG 2.x relative luminance of an sRGB color.
pub fn relative_luminance(color: &ThemeColor) -> f64 {
    let channel = |v: u8| -> f64 {
        let c = v as f64 / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let (r, g, b) = color.channels();
    0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b)
}

/// WCAG 2.x contrast ratio, 1.0 (identical) ..= 21.0 (black on white).
pub fn contrast_ratio(a: &ThemeColor, b: &ThemeColor) -> f64 {
    let la = relative_luminance(a);
    let lb = relative_luminance(b);
    let (hi, lo) = if la >= lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

/// WCAG AA thresholds for normal-size text.
pub fn meets_aa(ratio: f64, large_text: bool) -> bool {
    if large_text { ratio >= 3.0 } else { ratio >= 4.5 }
}

/// WCAG AAA thresholds for normal-size text.
pub fn meets_aaa(ratio: f64, large_text: bool) -> bool {
    if large_text { ratio >= 4.5 } else { ratio >= 7.0 }
}

/// A complete theme profile — everything Orbit needs to render, nothing executable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeProfile {
    pub id: String,
    pub name: String,
    /// Accent used across Orbit surfaces.
    pub accent: ThemeColor,
    pub dark: bool,
    /// Palette roles, e.g. {"surface": "#16181d", "text": "#e6e6e6"}. All validated colors.
    pub palette: Vec<(String, ThemeColor)>,
    /// Optional wallpaper reference — a data URL or a file path on disk, never a URL
    /// that Orbit would fetch on its own.
    pub wallpaper: Option<String>,
    pub icon_theme: Option<String>,
}

impl ThemeProfile {
    pub fn new(id: impl Into<String>, name: impl Into<String>, accent: ThemeColor, dark: bool) -> Result<Self, ThemePortError> {
        let id = id.into();
        validate_slug(&id)?;
        let name = name.into();
        if name.trim().is_empty() {
            return Err(ThemePortError::BadId("(empty name)".into()));
        }
        Ok(Self { id, name, accent, dark, palette: Vec::new(), wallpaper: None, icon_theme: None })
    }

    /// Audit the palette for unreadable text-on-surface pairs. Returns warnings, not errors.
    pub fn contrast_warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        let lookup = |key: &str| self.palette.iter().find(|(k, _)| k == key).map(|(_, c)| c);
        if let (Some(fg), Some(bg)) = (lookup("text"), lookup("surface")) {
            let ratio = contrast_ratio(fg, bg);
            if !meets_aa(ratio, false) {
                warnings.push(format!("text on surface contrast {ratio:.2} < 4.5 (WCAG AA)"));
            }
        }
        warnings
    }
}

fn validate_slug(id: &str) -> Result<(), ThemePortError> {
    let ok = !id.is_empty()
        && id.len() <= 64
        && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !id.starts_with('-')
        && !id.ends_with('-');
    if ok { Ok(()) } else { Err(ThemePortError::BadId(id.into())) }
}

fn validate_semver(version: &str) -> Result<(), ThemePortError> {
    let parts: Vec<&str> = version.split('.').collect();
    let ok = parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.len() <= 4 && p.chars().all(|c| c.is_ascii_digit()));
    if ok { Ok(()) } else { Err(ThemePortError::BadVersion(version.into())) }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackEntry {
    pub id: String,
    /// Relative path inside the pack archive, e.g. "themes/amber.json".
    pub file: String,
}

/// A signed-data-only theme pack manifest. `deny_unknown_fields` means any extra key
/// (notably anything script-like: `on_apply`, `exec`, `postinstall`, ...) fails parsing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackManifest {
    /// Manifest format version. Only 1 exists today.
    pub format: u32,
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub themes: Vec<PackEntry>,
}

impl PackManifest {
    pub fn validate(&self) -> Result<(), ThemePortError> {
        if self.format != 1 {
            return Err(ThemePortError::BadFormat(self.format));
        }
        validate_slug(&self.id)?;
        validate_semver(&self.version)?;
        if self.themes.is_empty() {
            return Err(ThemePortError::EmptyPack);
        }
        let mut seen = BTreeSet::new();
        for t in &self.themes {
            validate_slug(&t.id)?;
            if !seen.insert(t.id.as_str()) {
                return Err(ThemePortError::DuplicateThemeId(t.id.clone()));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_parsing_normalizes_case() {
        assert_eq!(ThemeColor::parse("#AABBCC").unwrap().as_str(), "#aabbcc");
        assert_eq!(ThemeColor::parse("aabbcc").unwrap().as_str(), "#aabbcc");
        assert!(ThemeColor::parse("#abc").is_err());
        assert!(ThemeColor::parse("#gg0000").is_err());
    }

    #[test]
    fn black_on_white_is_21() {
        let black = ThemeColor::parse("#000000").unwrap();
        let white = ThemeColor::parse("#ffffff").unwrap();
        let ratio = contrast_ratio(&black, &white);
        assert!((ratio - 21.0).abs() < 1e-9, "ratio was {ratio}");
    }

    #[test]
    fn identical_colors_are_1() {
        let c = ThemeColor::parse("#6c8cff").unwrap();
        assert!((contrast_ratio(&c, &c) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn relative_luminance_white_is_one() {
        let white = ThemeColor::parse("#ffffff").unwrap();
        assert!((relative_luminance(&white) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn aa_thresholds() {
        assert!(meets_aa(4.5, false));
        assert!(!meets_aa(4.49, false));
        assert!(meets_aa(3.0, true));
        assert!(meets_aaa(7.0, false));
    }

    #[test]
    fn contrast_warnings_catch_low_contrast() {
        let accent = ThemeColor::parse("#6c8cff").unwrap();
        let mut profile = ThemeProfile::new("test", "Test", accent, true).unwrap();
        profile.palette = vec![
            ("surface".into(), ThemeColor::parse("#000000").unwrap()),
            ("text".into(), ThemeColor::parse("#111111").unwrap()),
        ];
        let warnings = profile.contrast_warnings();
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn manifest_roundtrip_and_validation() {
        let m = PackManifest {
            format: 1,
            id: "amber-night".into(),
            name: "Amber Night".into(),
            version: "1.2.3".into(),
            author: Some("Orbit contributors".into()),
            description: None,
            themes: vec![PackEntry { id: "amber-dark".into(), file: "themes/amber-dark.json".into() }],
        };
        m.validate().unwrap();
        let json = serde_json::to_string(&m).unwrap();
        let back: PackManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(back, m);
    }

    #[test]
    fn manifest_rejects_script_and_junk_fields() {
        let json = r#"{"format":1,"id":"x","name":"X","version":"1.0.0","themes":[{"id":"t","file":"t.json"}],"on_apply":"rm -rf /"}"#;
        assert!(serde_json::from_str::<PackManifest>(json).is_err());
    }

    #[test]
    fn manifest_rejects_duplicate_theme_ids() {
        let m = PackManifest {
            format: 1,
            id: "x".into(),
            name: "X".into(),
            version: "1.0.0".into(),
            author: None,
            description: None,
            themes: vec![
                PackEntry { id: "t".into(), file: "a.json".into() },
                PackEntry { id: "t".into(), file: "b.json".into() },
            ],
        };
        assert_eq!(m.validate().unwrap_err(), ThemePortError::DuplicateThemeId("t".into()));
    }

    #[test]
    fn manifest_rejects_bad_version_and_format() {
        let bad = PackManifest {
            format: 2,
            id: "x".into(),
            name: "X".into(),
            version: "1.0".into(),
            author: None,
            description: None,
            themes: vec![PackEntry { id: "t".into(), file: "t.json".into() }],
        };
        assert_eq!(bad.validate().unwrap_err(), ThemePortError::BadFormat(2));

        let badver = PackManifest { format: 1, version: "1.0".into(), ..bad };
        assert!(matches!(badver.validate(), Err(ThemePortError::BadVersion(_))));
    }

    #[test]
    fn empty_pack_rejected() {
        let m = PackManifest {
            format: 1,
            id: "x".into(),
            name: "X".into(),
            version: "1.0.0".into(),
            author: None,
            description: None,
            themes: vec![],
        };
        assert_eq!(m.validate().unwrap_err(), ThemePortError::EmptyPack);
    }
}
