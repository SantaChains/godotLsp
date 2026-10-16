#!/usr/bin/env python3
"""拉取 Godot 官方 demo 工程语料到 corpus/godot-demo-projects/。

按仓库分发策略,未修改的上游工程(MIT)不入库;本脚本从官方仓库的
pinned tag 下载 tarball,只解出 .gd 文件(gen_freq.py 统计的唯一消费格式),
与 corpus/README.md 记录的 tag 一致。幂等:目录已存在时先删除再拉。

用法:
    uv run tools/fetch_demo.py
"""

from __future__ import annotations

import argparse
import shutil
import sys
import tarfile
import urllib.request
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
DEST = REPO_ROOT / "corpus" / "godot-demo-projects"
PINNED_TAG = "4.7-6ad6167"
URL = f"https://github.com/godotengine/godot-demo-projects/archive/refs/tags/{PINNED_TAG}.tar.gz"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--dest", default=str(DEST), help="输出目录(默认 corpus/godot-demo-projects)")
    args = ap.parse_args()
    dest = Path(args.dest).resolve()

    if dest.exists():
        print(f"[fetch] 删除已有 {dest}")
        shutil.rmtree(dest)
    dest.mkdir(parents=True)

    tgz = dest.parent / ".demo-tmp.tar.gz"
    try:
        print(f"[fetch] 下载 {URL}")
        req = urllib.request.Request(URL, headers={"User-Agent": "godotlsp-corpus/1.0"})
        with urllib.request.urlopen(req, timeout=120) as r, tgz.open("wb") as f:
            shutil.copyfileobj(r, f)
        n = 0
        with tarfile.open(tgz, "r:gz") as tf:
            for m in tf.getmembers():
                if m.isfile() and m.name.endswith(".gd"):
                    # 归一化落盘路径:去掉 tarball 顶层目录前缀,保留仓库相对路径。
                    rel = Path(m.name).relative_to(m.name.split("/")[0])
                    out = dest / rel
                    out.parent.mkdir(parents=True, exist_ok=True)
                    with tf.extractfile(m) as src, out.open("wb") as f:
                        shutil.copyfileobj(src, f)
                    n += 1
        print(f"[fetch] 解出 {n} 个 .gd → {dest}")
    except Exception as e:  # noqa: BLE001 — 网络失败要给出可读信息后退出
        print(f"[fetch] 失败: {e}", file=sys.stderr)
        # 失败清理半成品,gen_freq.py 对缺失目录走 SKIP 降级。
        if dest.exists():
            shutil.rmtree(dest)
        return 1
    finally:
        tgz.unlink(missing_ok=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
