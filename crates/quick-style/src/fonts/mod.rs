//! Embedded font data for Quick UI framework.
//!
//! Inter is licensed under the SIL Open Font License, Version 1.1.
//! Copyright 2020 The Inter Project Authors (https://github.com/rsms/inter)
//!
//! Embedding strategy:
//! - `InterVariable.ttf` — single variable-weight font (400–900), preferred on Skia.
//! - `Inter-Regular.otf` / `Inter-Medium.otf` / `Inter-SemiBold.otf` — static fallbacks.
//!
//! On macOS, Quick requests `-apple-system` from the Skia FontMgr and the OS
//! automatically serves SF Pro — no SF Pro bytes are bundled or distributed.

/// Inter Variable (all weights 400–900 in one file).
/// Preferred font for Skia rendering — register with `FontMgr::register_from_data`.
pub const INTER_VARIABLE: &[u8] = include_bytes!("InterVariable.ttf");

/// Inter Regular (weight 400) — static fallback.
pub const INTER_REGULAR: &[u8]  = include_bytes!("Inter-Regular.otf");

/// Inter Medium (weight 500) — static fallback.
pub const INTER_MEDIUM: &[u8]   = include_bytes!("Inter-Medium.otf");

/// Inter SemiBold (weight 600) — static fallback.
pub const INTER_SEMIBOLD: &[u8] = include_bytes!("Inter-SemiBold.otf");

/// Font family name constant — use when passing to Skia FontMgr.
pub const INTER_FAMILY: &str = "Inter";

/// All embedded font byte slices as a flat array, for batch registration.
pub const ALL_INTER_FONTS: &[(&str, &[u8])] = &[
    ("Inter Variable",  INTER_VARIABLE),
    ("Inter Regular",   INTER_REGULAR),
    ("Inter Medium",    INTER_MEDIUM),
    ("Inter SemiBold",  INTER_SEMIBOLD),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inter_variable_is_valid_ttf() {
        // TTF files start with 0x00 0x01 0x00 0x00 (sfVersion table tag)
        // or 0x4F 0x54 0x54 0x4F ("OTTO" = OTF with CFF)
        // Variable fonts are TTF, so check for TTF header
        assert!(INTER_VARIABLE.len() > 4, "font data must not be empty");
        let magic = &INTER_VARIABLE[..4];
        assert!(
            magic == b"\x00\x01\x00\x00" || magic == b"true" || magic == b"OTTO" || magic == b"ttcf",
            "unexpected font magic bytes: {:02x?}", magic
        );
    }

    #[test]
    fn test_inter_static_fonts_are_valid_otf() {
        for (name, data) in &[("Regular", INTER_REGULAR), ("Medium", INTER_MEDIUM), ("SemiBold", INTER_SEMIBOLD)] {
            assert!(data.len() > 1024, "{name} font data seems too small");
            let magic = &data[..4];
            assert!(
                magic == b"OTTO" || magic == b"\x00\x01\x00\x00",
                "{name}: unexpected magic: {:02x?}", magic
            );
        }
    }

    #[test]
    fn test_all_inter_fonts_array_complete() {
        assert_eq!(ALL_INTER_FONTS.len(), 4);
        for (name, data) in ALL_INTER_FONTS {
            assert!(!name.is_empty());
            assert!(!data.is_empty(), "{name} data is empty");
        }
    }
}
