//! 诊断工具：扫描 corpus/godot-engine-tests，定位系统性解析缺口。
//!
//! 用法（在 crates/gdscript-syntax 下）：
//!   cargo run --example diagnose            # 打印目录无错率 + 错误频率 Top N
//!   cargo run --example diagnose -- --top 40 --samples common,enum_values_in_dictionary
//!
//! 不修改任何源码，纯分析。

use std::collections::{BTreeMap, HashMap};
use std::io::Write;
use std::path::PathBuf;

use gdscript_syntax::parse;

fn corpus_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop(); // crates/gdscript-syntax
    p.pop(); // crates
    p.join("corpus").join("godot-engine-tests")
}

fn walk(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                walk(&p, out);
            } else if p.extension().map(|x| x == "gd").unwrap_or(false) {
                out.push(p);
            }
        }
    }
}

/// 返回形如 "analyzer/features" 的两级相对键。
fn two_level_key(root: &std::path::Path, f: &std::path::Path) -> String {
    let rel = f.strip_prefix(root).unwrap_or(f);
    let comps: Vec<_> = rel.components().collect();
    if comps.len() >= 2 {
        format!("{}/{}", comps[0].as_os_str().to_string_lossy(), comps[1].as_os_str().to_string_lossy())
    } else if comps.len() == 1 {
        comps[0].as_os_str().to_string_lossy().to_string()
    } else {
        "_root".to_string()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut top_n = 30usize;
    let mut sample_dirs: Vec<String> = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--top" => {
                if let Some(v) = args.get(i + 1) {
                    top_n = v.parse().unwrap_or(top_n);
                }
                i += 2;
            }
            "--samples" => {
                if let Some(v) = args.get(i + 1) {
                    sample_dirs = v.split(',').map(|s| s.trim().to_string()).collect();
                }
                i += 2;
            }
            _ => i += 1,
        }
    }

    let root = corpus_root();
    let mut files = Vec::new();
    walk(&root, &mut files);
    files.sort();

    let mut by_key: BTreeMap<String, (usize, usize)> = BTreeMap::new(); // key -> (total, clean)
    let mut msg_count: HashMap<String, usize> = HashMap::new();
    let mut total = 0usize;
    let mut clean = 0usize;
    let mut total_errs = 0usize;

    // 采样：key -> Vec<(file, errors)>
    let mut samples: HashMap<String, Vec<(PathBuf, Vec<gdscript_syntax::ParseError>)>> = HashMap::new();

    let mut prog = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("diagnose_progress.log")
        .unwrap();
    for f in &files {
        let src = std::fs::read_to_string(f).expect("read");
        let rel = f.strip_prefix(&root).unwrap_or(f);
        let _ = writeln!(prog, "PARSE {}", rel.display());
        let _ = prog.flush();
        let (_ast, errs) = parse(&src);
        let key = two_level_key(&root, f);
        let e = by_key.entry(key.clone()).or_insert((0, 0));
        e.0 += 1;
        total += 1;
        if errs.is_empty() {
            e.1 += 1;
            clean += 1;
        } else {
            total_errs += errs.len();
            for er in &errs {
                // 以 "消息主体（去掉具体标识符）" 归桶
                let bucket = normalize_msg(&er.message);
                *msg_count.entry(bucket).or_insert(0) += 1;
            }
            if sample_dirs.iter().any(|d| key.contains(d)) {
                let v = samples.entry(key.clone()).or_default();
                if v.len() < 6 {
                    v.push((f.clone(), errs));
                }
            }
        }
    }

    println!("=== 总览 ===");
    println!("总文件={}, 无错={} ({:.1}%), 总错误数={}", total, clean, clean as f64 / total as f64 * 100.0, total_errs);

    println!("\n=== 各级子目录无错率（两级键）===");
    for (k, (t, c)) in &by_key {
        let pct = *c as f64 / *t as f64 * 100.0;
        let flag = if pct < 50.0 { "  <-- 低" } else { "" };
        println!("  {:<34} {}/{} ({:5.1}%){}", k, c, t, pct, flag);
    }

    println!("\n=== 错误消息频率 Top {} ===", top_n);
    let mut ranked: Vec<(String, usize)> = msg_count.into_iter().collect();
    ranked.sort_unstable_by_key(|(_, n)| std::cmp::Reverse(*n));
    for (msg, n) in ranked.iter().take(top_n) {
        println!("  {:>5}  {}", n, msg);
    }

    if !samples.is_empty() {
        println!("\n=== 采样文件错误详情 ===");
        for (k, v) in &samples {
            println!("# 目录: {}", k);
            for (f, errs) in v {
                let rel = f.strip_prefix(&root).unwrap_or(f);
                println!("  -- {}  ({} errs)", rel.display(), errs.len());
                for er in errs.iter().take(12) {
                    println!("       L{}: {}", er.line, er.message);
                }
            }
        }
    }
}

/// 把消息里的具体名字/数字归一化，便于归桶统计高频模式。
fn normalize_msg(s: &str) -> String {
    regex_lite(s)
}

// 极简归一化：把形如 `xxx 'name'`/`xxx "name"` 的引用名换成 <name>，
// 把行内数字换成 <n>，避免同一类错误因标识符不同而分散。
fn regex_lite(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    let mut in_quote: Option<char> = None;
    while let Some(c) = chars.next() {
        match c {
            '\'' | '"' => {
                if in_quote == Some(c) {
                    out.push_str("<name>");
                    in_quote = None;
                } else if in_quote.is_none() {
                    in_quote = Some(c);
                    out.push_str("<name>");
                } else {
                    out.push(c);
                }
            }
            _ if c.is_ascii_digit() => {
                // 吞掉连续数字
                if !out.ends_with("<n>") {
                    out.push_str("<n>");
                }
                while chars.peek().map(|x| x.is_ascii_digit()).unwrap_or(false) {
                    chars.next();
                }
            }
            _ => out.push(c),
        }
    }
    out
}
