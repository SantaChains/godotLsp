//! bench 语料解析零错误回归门槛：corpus/bench/ 下的真实大型工程
//! (Pixelorama/Dialogic/gdUnit4,pinned)必须全部无解析错误。
//! 语料缺失时 SKIP。曾抓出:not-in 缺失、括号内 lambda 结构丢失、
//! 内联 else/elif、字典内 lambda 等六类真实语法缺口。

use std::path::PathBuf;

#[test]
fn bench_corpus_parses_clean() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../corpus/bench");
    if !root.is_dir() {
        eprintln!("SKIP: bench 语料不存在(运行 python tools/fetch_bench.py 拉取)");
        return;
    }
    let mut files = 0usize;
    let mut failures: Vec<String> = Vec::new();
    walk(&root, &mut |p, src| {
        files += 1;
        let (_, errs) = gdscript_syntax::parse(src);
        if !errs.is_empty() {
            let shown: Vec<String> = errs
                .iter()
                .take(3)
                .map(|e| {
                    let line =
                        src[..e.start.min(src.len())].bytes().filter(|&c| c == b'\n').count() + 1;
                    format!("L{}: {}", line, e.message)
                })
                .collect();
            failures.push(format!("{}: {} errs ({})", p.display(), errs.len(), shown.join("; ")));
        }
    });
    assert!(files > 0, "bench 语料目录存在但没有 .gd 文件");
    assert!(
        failures.is_empty(),
        "{} 个文件存在解析错误(共 {} 个文件):\n{}",
        failures.len(),
        files,
        failures.join("\n")
    );
}

fn walk(dir: &std::path::Path, f: &mut dyn FnMut(&std::path::Path, &str)) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(&p, f);
        } else if p.extension().map(|x| x == "gd").unwrap_or(false)
            && let Ok(s) = std::fs::read_to_string(&p) {
                f(&p, &s);
            }
    }
}
