//! corpus 驱动测试：验证解析器「零 panic、错误有界」，并测量 Godot 上游语料的解析通过率。
//!
//! 设计见 `docs/from-scratch-design.md` §9。

use gdscript_syntax::parse;

fn corpus_root() -> std::path::PathBuf {
    let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop(); // crates/gdscript-syntax
    p.pop(); // crates
    p.join("corpus")
}

fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
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

#[test]
fn feature_showcase_parses_clean() {
    let root = corpus_root().join("feature-showcase");
    let mut files = Vec::new();
    walk(&root, &mut files);
    assert!(!files.is_empty(), "feature-showcase 为空");
    let mut failures = Vec::new();
    for f in &files {
        let src = std::fs::read_to_string(f).expect("read");
        let (_ast, errs) = parse(&src);
        if !errs.is_empty() {
            failures.push((f.clone(), errs));
        }
    }
    for (f, errs) in &failures {
        eprintln!("FAIL {}: {} errors", f.display(), errs.len());
        for e in errs.iter().take(3) {
            eprintln!("   L{}: {}", e.line, e.message);
        }
    }
    assert!(
        failures.is_empty(),
        "feature-showcase 应 100% 无错，实际 {} 个文件有错",
        failures.len()
    );
}

#[test]
fn godot_corpus_parser_is_panic_free_and_bounded() {
    let root = corpus_root().join("godot-engine-tests");
    let mut files = Vec::new();
    walk(&root, &mut files);
    let total = files.len();
    if total == 0 {
        // godot-engine-tests 是 Godot 上游(MIT)未修改测试夹具,按仓库分发策略不入库;
        // `python tools/fetch_corpus.py` 可按 pinned commit 拉取。缺语料时跳过而非失败,
        // 保证 fresh clone 的 `cargo test` 直接可跑(feature-showcase 夹具已入库,硬断言)。
        eprintln!("SKIP: corpus/godot-engine-tests 不存在(运行 python tools/fetch_corpus.py 拉取)");
        return;
    }

    let mut ok = 0usize;
    let mut errors_total = 0usize;
    let mut max_errs = 0usize;
    let mut by_dir: std::collections::BTreeMap<String, (usize, usize)> =
        std::collections::BTreeMap::new();

    for f in &files {
        let raw = std::fs::read_to_string(f).expect("read");
        // Godot 上游 completion 夹具用 U+27A1(➡) 标记光标位置，它不是 GDScript 语法；
        // 测试框架在解析前剥离游标标记。对齐该约定：剥离后再 parse。
        let src: String = raw.chars().filter(|&c| c != '\u{27a1}').collect();
        let (_ast, errs) = parse(&src); // 若 panic，测试即失败——这正是我们要抓住的
        errors_total += errs.len();
        max_errs = max_errs.max(errs.len());
        if errs.is_empty() {
            ok += 1;
        }
        let dir = f.parent()
            .and_then(|p| p.file_name())
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let entry = by_dir.entry(dir).or_insert((0, 0));
        entry.0 += 1;
        if errs.is_empty() {
            entry.1 += 1;
        }
    }

    let rate = ok as f64 / total as f64;
    eprintln!(
        "godot-engine-tests: {}/{} 无错 ({:.1}%), 总错误数={}, 单文件最大错误数={}",
        ok, total, rate * 100.0, errors_total, max_errs
    );
    for (dir, (t, o)) in &by_dir {
        eprintln!("  {:<12} {}/{} 无错", dir, o, t);
    }

    // 硬指标 1：零 panic（已由 parse 保证，崩溃即测试失败）。
    // 硬指标 2：错误恢复有界——任何单文件错误数不超过 500（捕获失控的级联报错）。
    assert!(
        max_errs < 500,
        "存在单文件错误数 {} 过高，疑似错误恢复失控",
        max_errs
    );
}
