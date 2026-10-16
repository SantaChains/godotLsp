#!/usr/bin/env python3
"""拉取大型真实 Godot 工程(基准语料)到 corpus/bench/<name>/。

按仓库分发策略,未修改的上游工程(MIT)不入库;本脚本从各仓库的
pinned commit 下载完整 tarball 解出(不同于 fetch_demo.py 只解 .gd:
负载测试需要完整工程——project.godot/autoload、.tscn、.tres 都参与
workspace 解析)。与 corpus/README.md 记录的 commit 一致。
幂等:目录已存在时先删除再拉,保证可重复刷新。

用法:
    uv run tools/fetch_bench.py [--only pixelorama]
"""

from __future__ import annotations

import argparse
import shutil
import sys
import tarfile
import urllib.request
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
DEST_ROOT = REPO_ROOT / "corpus" / "bench"

# pinned 2026-10-16(HEAD);更新 pin 时同步改 corpus/README.md。
REPOS: dict[str, tuple[str, str]] = {
    "pixelorama": (
        "Orama-Interactive/Pixelorama",
        "e64b4762bf0462757f9fd6a15caf7039f3035107",
    ),
    "dialogic": (
        "dialogic-godot/dialogic",
        "701b67bbe60114e010476d1685f7de1261091c29",
    ),
    "gdunit4": (
        "MikeSchulze/gdUnit4",
        "306eb30f8f922a2e841dbd35e322cbc13f512377",
    ),
}


def fetch_one(name: str, repo: str, sha: str) -> int:
    dest = DEST_ROOT / name
    if dest.exists():
        print(f"[fetch] 删除已有 {dest}")
        shutil.rmtree(dest)
    dest.mkdir(parents=True)

    url = f"https://github.com/{repo}/archive/{sha}.tar.gz"
    tgz = dest.parent / f".{name}-tmp.tar.gz"
    try:
        print(f"[fetch] 下载 {url}")
        req = urllib.request.Request(url, headers={"User-Agent": "godotlsp-bench/1.0"})
        with urllib.request.urlopen(req, timeout=300) as r, tgz.open("wb") as f:
            shutil.copyfileobj(r, f)
        with tarfile.open(tgz, "r:gz") as tf:
            # 归一化落盘路径:去掉 tarball 顶层目录前缀,保留仓库相对路径。
            tf.extractall(dest, filter="data")
        top = next(dest.iterdir())
        if top.is_dir():
            for child in top.iterdir():
                shutil.move(str(child), dest / child.name)
            top.rmdir()

        n_gd = sum(1 for _ in dest.rglob("*.gd"))
        n_tscn = sum(1 for _ in dest.rglob("*.tscn"))
        ok = (dest / "project.godot").is_file()
        print(
            f"[fetch] {name}: {n_gd} 个 .gd / {n_tscn} 个 .tscn / "
            f"project.godot={'有' if ok else '无'} → {dest}"
        )
        return 0
    except Exception as e:  # noqa: BLE001 — 网络失败要给出可读信息后退出
        print(f"[fetch] {name} 失败: {e}", file=sys.stderr)
        if dest.exists():
            shutil.rmtree(dest)
        return 1
    finally:
        tgz.unlink(missing_ok=True)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--only", choices=sorted(REPOS), help="只拉取单个工程")
    args = ap.parse_args()

    DEST_ROOT.mkdir(parents=True, exist_ok=True)
    rc = 0
    for name, (repo, sha) in sorted(REPOS.items()):
        if args.only and name != args.only:
            continue
        rc |= fetch_one(name, repo, sha)
    return rc


if __name__ == "__main__":
    raise SystemExit(main())
