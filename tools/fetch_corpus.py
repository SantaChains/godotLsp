#!/usr/bin/env python3
"""拉取 Godot 上游 GDScript 测试语料到 corpus/godot-engine-tests/。

按仓库分发策略,未修改的 Godot 上游夹具(MIT)不入库;本脚本从官方仓库的
pinned commit 稀疏拉取 `modules/gdscript/tests/scripts`,与 corpus/README.md
记录的 commit 一致。幂等:目录已存在时先删除再拉,保证可重复刷新。

用法:
    uv run tools/fetch_corpus.py
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
DEST = REPO_ROOT / "corpus" / "godot-engine-tests"
PINNED_COMMIT = "9a05d575f9f5b9dc8f0d5681028089900a73aa80"
SUBDIR = "modules/gdscript/tests/scripts"


def run(cmd: list[str], **kw: object) -> None:
    print(f"[fetch] {' '.join(cmd)}")
    subprocess.run(cmd, check=True, **kw)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--dest", default=str(DEST), help="输出目录(默认 corpus/godot-engine-tests)")
    args = ap.parse_args()
    dest = Path(args.dest).resolve()

    if dest.exists():
        print(f"[fetch] 删除已有 {dest}")
        shutil.rmtree(dest)
    dest.parent.mkdir(parents=True, exist_ok=True)

    # partial clone + sparse checkout:只取 tests/scripts 子目录,省 90% 以上流量。
    tmp = dest.parent / ".corpus-tmp"
    if tmp.exists():
        shutil.rmtree(tmp)
    try:
        run(["git", "init", str(tmp)])
        run(["git", "-C", str(tmp), "remote", "add", "origin",
             "https://github.com/godotengine/godot.git"])
        run(["git", "-C", str(tmp), "config", "extensions.partialClone", "origin"])
        run(["git", "-C", str(tmp), "sparse-checkout", "set", SUBDIR])
        run(["git", "-C", str(tmp), "fetch", "--depth", "1", "--filter=blob:none",
             "origin", PINNED_COMMIT])
        run(["git", "-C", str(tmp), "checkout", "FETCH_HEAD"])
        src = tmp / SUBDIR
        if not src.is_dir():
            print(f"[fetch] 错误:上游 commit 中不存在 {SUBDIR}", file=sys.stderr)
            return 2
        shutil.copytree(src, dest)
    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    n = sum(1 for p in dest.rglob("*.gd"))
    print(f"[fetch] 完成:{dest} 共 {n} 个 .gd 文件(pinned {PINNED_COMMIT[:12]})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
