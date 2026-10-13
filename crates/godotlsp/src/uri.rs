//! `file:` URI 与文件系统路径的双向转换,含百分号编解码。
//!
//! 改编自 `gdscript-analyzer` 的 `gdscript-lsp`(MIT OR Apache-2.0),
//! 同样解决了 lsp-types 0.97 的 `Uri` 迁移问题。

use std::path::{Path, PathBuf};

use lsp_types::Uri;

/// `file:` [`Uri`] 转文件系统路径:百分号解码,并剥去 Windows 盘符前的斜杠。非 `file:` URI 返回 `None`。
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

/// 文件系统路径转 `file:` [`Uri`]:归一化分隔符,百分号编码保留字符。
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

/// 百分号解码 URI 路径片段(`%XX` → 字节);畸形转义原样保留。
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

/// 路径的百分号编码:保留非保留字符集 + `/`(分隔符)+ `:`(Windows 盘符),其余编码。
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

/// 十六进制位取值(`0-9`/`a-f`/`A-F`),否则 `None`。
fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
