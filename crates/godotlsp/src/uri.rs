//! `file:` URI ↔ filesystem path conversion, percent-encoding aware.
//!
//! Adapted from `gdscript-analyzer`'s `gdscript-lsp` (MIT OR Apache-2.0), which solved the
//! same lsp-types 0.97 `Uri` migration.

use std::path::{Path, PathBuf};

use lsp_types::Uri;

/// Convert a `file:` [`Uri`] to a filesystem path, percent-decoding and stripping the
/// leading slash before a Windows drive. `None` for non-`file:` URIs.
#[must_use]
pub fn uri_to_path(uri: &Uri) -> Option<PathBuf> {
    let rest = uri.as_str().strip_prefix("file://")?;
    let path_part = if rest.starts_with('/') {
        rest
    } else {
        &rest[rest.find('/')?..]
    };
    let decoded = percent_decode(path_part);
    let bytes = decoded.as_bytes();
    let trimmed = if bytes.len() >= 3 && bytes[0] == b'/' && bytes[2] == b':' {
        &decoded[1..]
    } else {
        decoded.as_str()
    };
    Some(PathBuf::from(trimmed))
}

/// Convert a filesystem path to a `file:` [`Uri`], normalizing separators and
/// percent-encoding reserved characters.
#[must_use]
pub fn path_to_uri(path: &Path) -> Option<Uri> {
    let mut s = path.to_string_lossy().replace('\\', "/");
    if s.starts_with("//?/") {
        s.drain(.."//?/".len());
    }
    let with_root = if s.starts_with('/') {
        s
    } else {
        format!("/{s}")
    };
    format!("file://{}", percent_encode_path(&with_root))
        .parse()
        .ok()
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 3 <= bytes.len()
            && let (Some(h), Some(l)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2]))
        {
            out.push(h * 16 + l);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn percent_encode_path(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        let keep =
            b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~' | b'/' | b':');
        if keep {
            out.push(b as char);
        } else {
            out.push('%');
            out.push(char::from_digit(u32::from(b >> 4), 16).unwrap().to_ascii_uppercase());
            out.push(char::from_digit(u32::from(b & 0xf), 16).unwrap().to_ascii_uppercase());
        }
    }
    out
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
