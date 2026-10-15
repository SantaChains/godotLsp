#!/usr/bin/env python3
"""Watcher-path probe: does a mid-session project.godot autoload addition take effect?

Sequence: initialize(root) -> didOpen(main.gd) [expect LateSingleton error]
-> modify project.godot on disk (add LateSingleton autoload)
-> didChangeWatchedFiles(project.godot)
-> didChange(main.gd, no-op edit) to force re-diagnosis
-> expect: LateSingleton error cleared.
"""
import json
import subprocess
import sys
import threading
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from lsp_probe import make_message, read_messages  # noqa: E402


def main():
    exe, proj = sys.argv[1], Path(sys.argv[2])
    proc = subprocess.Popen(
        [exe], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL
    )
    msgs = []
    threading.Thread(target=read_messages, args=(proc.stdout, msgs), daemon=True).start()

    def send(obj):
        proc.stdin.write(make_message(obj))
        proc.stdin.flush()

    main_uri = (proj / "main.gd").resolve().as_uri()
    proj_uri = (proj / "project.godot").resolve().as_uri()

    send({"jsonrpc": "2.0", "id": 1, "method": "initialize",
          "params": {"processId": None, "rootUri": proj.resolve().as_uri(), "capabilities": {}}})
    time.sleep(0.5)
    send({"jsonrpc": "2.0", "method": "initialized", "params": {}})
    time.sleep(1.5)
    text = (proj / "main.gd").read_text(encoding="utf-8")
    send({"jsonrpc": "2.0", "method": "textDocument/didOpen",
          "params": {"textDocument": {"uri": main_uri, "languageId": "gdscript",
                                      "version": 1, "text": text}}})
    time.sleep(3)

    def last_diags():
        d = [m for m in msgs if m.get("method") == "textDocument/publishDiagnostics"]
        return d[-1]["params"]["diagnostics"] if d else None

    print("PHASE1 diags:", json.dumps(last_diags(), ensure_ascii=False))

    # Disk-side change: add the second autoload.
    cfg = (proj / "project.godot").read_text(encoding="utf-8")
    if "LateSingleton" not in cfg:
        cfg = cfg.rstrip("\n") + '\nLateSingleton="*res://scripts/late.gd"\n'
        (proj / "project.godot").write_text(cfg, encoding="utf-8")
    (proj / "scripts").mkdir(exist_ok=True)
    (proj / "scripts" / "late.gd").write_text("extends Node\nvar late: int = 1\n", encoding="utf-8")

    send({"jsonrpc": "2.0", "method": "workspace/didChangeWatchedFiles",
          "params": {"changes": [{"uri": proj_uri, "type": 2}]}})
    time.sleep(2)
    print("PHASE2 (after watcher event, no doc touch) diags:",
          json.dumps(last_diags(), ensure_ascii=False))

    # Force re-diagnosis via a no-op didChange.
    send({"jsonrpc": "2.0", "method": "textDocument/didChange",
          "params": {"textDocument": {"uri": main_uri, "version": 2},
                     "contentChanges": [{"text": text}]}})
    time.sleep(3)
    print("PHASE3 (after no-op didChange) diags:",
          json.dumps(last_diags(), ensure_ascii=False))
    proc.kill()


if __name__ == "__main__":
    main()
