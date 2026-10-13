//! 纯文本层特性:增量编辑应用、`res://` 文档链接、`Color()` 取色、`##` 文档注释回扫。
//! 全部只依赖文档文本与路径,不经分析引擎。

use gdscript_base::TextRange;
use lsp_types::{Color, ColorInformation, ColorPresentation, DocumentLink, TextDocumentContentChangeEvent, Uri};
use std::path::Path;

use crate::convert::LineIndex;
use crate::uri::path_to_uri;

/// 依序把 `didChange` 内容变更应用到文档:增量 range 编辑(UTF-16)或全文替换。
///
/// 每个 change 相对前一版本文本,因此逐个应用、每个之后重建行索引,
/// 供下一个 change 的位置转换使用(与 LSP 规范一致)。
pub fn apply_content_changes(
    text: &mut String,
    index: &mut LineIndex,
    changes: Vec<TextDocumentContentChangeEvent>,
) {
    for c in changes {
        match c.range {
            Some(r) => {
                let start = index.offset(text, r.start) as usize;
                let end = index.offset(text, r.end) as usize;
                let (lo, hi) = (start.min(end).min(text.len()), start.max(end).min(text.len()));
                text.replace_range(lo..hi, &c.text);
            }
            None => *text = c.text,
        }
        *index = LineIndex::new(text);
    }
}

/// 扫描文本中的 `"res://…"` / `'res://…'` 字符串字面量,生成可点击的文档链接。
/// 目标指向项目根下解析出的文件;文件不存在时逐段回退到最长存在的前缀目录。
pub fn document_links(text: &str, root: &Path) -> Vec<DocumentLink> {
    let bytes = text.as_bytes();
    let index = LineIndex::new(text);
    let mut out = Vec::new();
    let mut i = 0usize;
    while let Some(pos) = text[i..].find("res://") {
        let start = i + pos;
        // 链接只作用于字符串字面量:紧邻的前一字符必须是引号。
        let Some(quote) = start.checked_sub(1).and_then(|p| bytes.get(p).copied()).filter(|&b| b == b'"' || b == b'\'') else {
            i = start + 6;
            continue;
        };
        let Some(end) = bytes[start..].iter().position(|&b| b == quote).map(|p| start + p) else {
            break; // 未闭合,后面也不会再有
        };
        let rel = &text[start..end];
        #[allow(clippy::cast_possible_truncation)]
        let range = TextRange::new(start as u32, end as u32);
        if let Some(target) = resolve_res(root, rel) {
            out.push(DocumentLink {
                range: index.range(text, range),
                target: Some(target),
                tooltip: None,
                data: None,
            });
        }
        i = end + 1;
    }
    out
}

/// 把 `res://` 资源路径解析为文件 URI;不存在则回退到最长存在的前缀目录。
fn resolve_res(root: &Path, res: &str) -> Option<Uri> {
    let rel = res.strip_prefix("res://")?;
    if rel.is_empty() {
        return None;
    }
    let full = root.join(rel);
    if full.is_file() {
        return path_to_uri(&full);
    }
    let mut cur = root.to_path_buf();
    for seg in rel.split('/') {
        cur.push(seg);
        if !cur.exists() {
            cur.pop();
            break;
        }
    }
    (cur != root).then(|| path_to_uri(&cur)).flatten()
}

/// 扫描 `Color(...)` 构造调用,产出颜色信息(编辑器色板取色器)。
/// 支持 `Color("#rrggbb[aa]")`、`Color("rrggbb[aa]")` 与 `Color(r, g, b[, a])`(0..1 浮点)。
pub fn document_colors(text: &str) -> Vec<ColorInformation> {
    let index = LineIndex::new(text);
    let mut out = Vec::new();
    let mut i = 0usize;
    while let Some(pos) = text[i..].find("Color(") {
        let start = i + pos;
        let Some(close) = text[start..].find(')') else {
            break;
        };
        let end = start + close + 1;
        let inner = &text[start + "Color(".len()..start + close];
        if let Some(color) = parse_color_args(inner) {
            #[allow(clippy::cast_possible_truncation)]
            let range = TextRange::new(start as u32, end as u32);
            out.push(ColorInformation {
                range: index.range(text, range),
                color,
            });
        }
        i = end;
    }
    out
}

/// 解析 `Color(` 的实参列表为颜色。
fn parse_color_args(inner: &str) -> Option<Color> {
    let inner = inner.trim();
    let first = inner.as_bytes().first().copied()?;
    if first == b'"' || first == b'\'' {
        let end = inner[1..].bytes().position(|b| b == first)? + 1;
        return parse_hex(&inner[1..end]);
    }
    let mut nums = [0f32; 4];
    let mut n = 0usize;
    for part in inner.split(',') {
        let v: f32 = part.trim().parse().ok()?;
        *nums.get_mut(n)? = v;
        n += 1;
    }
    if n < 3 {
        return None;
    }
    let a = if n == 4 { nums[3] } else { 1.0 };
    Some(Color {
        red: clamp01(nums[0]),
        green: clamp01(nums[1]),
        blue: clamp01(nums[2]),
        alpha: clamp01(a),
    })
}

/// 解析 `#rrggbb[aa]`(前导 `#` 可省略)为颜色。
fn parse_hex(s: &str) -> Option<Color> {
    let s = s.strip_prefix('#').unwrap_or(s);
    let (rgb, alpha) = match s.len() {
        6 => (s, 1.0),
        8 => {
            let a = u8::from_str_radix(&s[6..8], 16).ok()?;
            (s, f32::from(a) / 255.0)
        }
        _ => return None,
    };
    Some(Color {
        red: u8::from_str_radix(&rgb[..2], 16).ok()? as f32 / 255.0,
        green: u8::from_str_radix(&rgb[2..4], 16).ok()? as f32 / 255.0,
        blue: u8::from_str_radix(&rgb[4..6], 16).ok()? as f32 / 255.0,
        alpha,
    })
}

fn clamp01(v: f32) -> f32 {
    v.clamp(0.0, 1.0)
}

/// 为颜色生成可选的文本表示:`Color(r, g, b, a)` 浮点式与 `Color("#hex")` 十六进制式。
pub fn color_presentations(color: &Color) -> Vec<ColorPresentation> {
    let f = |v: f32| {
        let s = format!("{:.3}", v);
        s.trim_end_matches('0').trim_end_matches('.').to_owned()
    };
    let hex = |v: f32| format!("{:02x}", (clamp01(v) * 255.0).round() as u8);
    let edit = |label: String| ColorPresentation {
        label,
        text_edit: None,
        additional_text_edits: None,
    };
    vec![
        edit(format!(
            "Color({}, {}, {}, {})",
            f(color.red),
            f(color.green),
            f(color.blue),
            f(color.alpha)
        )),
        edit(format!(
            "Color(\"{}{}{}{}\")",
            hex(color.red),
            hex(color.green),
            hex(color.blue),
            hex(color.alpha)
        )),
    ]
}

/// 从 `start_line` 上一行起向上回扫连续的 `##` 文档注释,拼成 Markdown。
/// 中途跳过注解行(`@…`,文档规范允许注释与成员之间隔注解),遇到其他内容即停。
pub fn doc_comment_above(text: &str, index: &LineIndex, start_line: u32) -> Option<String> {
    let mut lines = Vec::new();
    let mut l = start_line;
    while l > 0 {
        l -= 1;
        let (s, e) = index.line_span(l);
        let t = text[s..e].trim_start();
        if let Some(body) = t.strip_prefix("##") {
            lines.push(body.trim().to_owned());
        } else if t.starts_with('@') {
            continue; // 注解行:文档可置于注解之上
        } else {
            break;
        }
    }
    if lines.is_empty() {
        return None;
    }
    lines.reverse();
    Some(lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lsp_types::Position;

    fn idx(text: &str) -> LineIndex {
        LineIndex::new(text)
    }

    /// 测试用范围构造器。
    fn r(sl: u32, sc: u32, el: u32, ec: u32) -> lsp_types::Range {
        lsp_types::Range {
            start: Position {
                line: sl,
                character: sc,
            },
            end: Position {
                line: el,
                character: ec,
            },
        }
    }

    /// 单条增量编辑的测试构造器。
    fn edit(range: Option<lsp_types::Range>, text: &str) -> TextDocumentContentChangeEvent {
        TextDocumentContentChangeEvent {
            range,
            range_length: None,
            text: text.to_owned(),
        }
    }

    #[test]
    fn incremental_insert_and_delete() {
        let mut text = "func a():\n\tpass\n".to_owned();
        let mut index = idx(&text);
        // 在第一行行尾追加 `x`(UTF-16 位置)。
        apply_content_changes(&mut text, &mut index, vec![edit(Some(r(0, 9, 0, 9)), "x")]);
        assert_eq!(text, "func a():x\n\tpass\n");
        // 删除刚才追加的 `x`。
        apply_content_changes(&mut text, &mut index, vec![edit(Some(r(0, 9, 0, 10)), "")]);
        assert_eq!(text, "func a():\n\tpass\n");
        // 全文替换。
        apply_content_changes(&mut text, &mut index, vec![edit(None, "var q = 1\n")]);
        assert_eq!(text, "var q = 1\n");
    }

    #[test]
    fn multi_change_sequence() {
        let mut text = "var a = 1\nvar b = 2\n".to_owned();
        let mut index = idx(&text);
        apply_content_changes(
            &mut text,
            &mut index,
            vec![edit(Some(r(0, 4, 0, 5)), "aa"), edit(Some(r(1, 4, 1, 5)), "bb")],
        );
        assert_eq!(text, "var aa = 1\nvar bb = 2\n");
    }

    #[test]
    fn links_point_at_files_and_fall_back() {
        let root = std::env::temp_dir().join(format!("godotlsp_links_{}", std::process::id()));
        std::fs::create_dir_all(root.join("res")).unwrap();
        std::fs::write(root.join("scene.tscn"), "").unwrap();
        let text = "const A = preload(\"res://scene.tscn\")\nconst B = 'res://res/missing/'\nconst C = \"no link\"\n";
        let links = document_links(text, &root);
        assert_eq!(links.len(), 2, "{links:?}");
        let t0 = links[0].target.as_ref().unwrap().as_str();
        assert!(t0.ends_with("scene.tscn"), "{t0}");
        // 不存在的文件回退到最长存在目录。
        let t1 = links[1].target.as_ref().unwrap().as_str();
        assert!(t1.ends_with("res"), "{t1}");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn colors_parse_hex_and_rgba() {
        let text = "var a = Color(\"#ff8000\")\nvar b = Color(1, 0.5, 0, 0.8)\nvar c = Color(\"bad\")\n";
        let colors = document_colors(text);
        assert_eq!(colors.len(), 2, "{colors:?}");
        assert!((colors[0].color.red - 1.0).abs() < 1e-6);
        assert!((colors[0].color.green - 128.0 / 255.0).abs() < 1e-6);
        assert!((colors[1].color.alpha - 0.8).abs() < 1e-6);
    }

    #[test]
    fn color_presentations_round_trip() {
        let c = Color { red: 1.0, green: 0.5, blue: 0.0, alpha: 1.0 };
        let ps = color_presentations(&c);
        assert_eq!(ps[0].label, "Color(1, 0.5, 0, 1)");
        assert_eq!(ps[1].label, "Color(\"ff8000ff\")");
    }

    #[test]
    fn doc_comment_skips_annotations() {
        let text = "## 商人的血量。\n@export\nvar hp := 10\n";
        let index = idx(text);
        let md = doc_comment_above(text, &index, 2).unwrap();
        assert_eq!(md, "商人的血量。");
        // 普通行截断回扫。
        let text2 = "var other := 1\nvar hp := 10\n";
        assert_eq!(doc_comment_above(text2, &idx(text2), 1), None);
    }
}
