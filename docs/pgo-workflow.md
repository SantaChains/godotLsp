# PGO release 构建工作流(cargo-make)

## 结论先行:BOLT 在 Windows 不可行

- **BOLT 官方不支持 PE/COFF**。LLVM Discourse 上 BOLT 维护者(aaupov)明确:"BOLT team has no plan to add it"。
  前提条件全部缺失:链接器 `--emit-relocs` 的 PE 等价物、LBR/ETW profile 到 BOLT YAML 的转换、FDE 级函数边界机制。
- 本机 scoop LLVM 23.1.3(93 个工具)**不含 llvm-bolt.exe**;LLVM 官方 Windows 安装包同样不含,GitHub release 只有 `bolt-*.src.tar.xz` 源码。
- **等价替代 = PGO**:代码布局/内联/分支优化交给 LLVM 基于 profile 决策,收益方向与 BOLT 相同(热代码聚簇、icache 命中)。Windows 原生全链路可行,本机已端到端验证。

## 用法

```bash
cargo make pgo        # 全流程:clean → 插桩构建 → 跑负载 → merge → PGO 优化构建
cargo make pgo-build  # 只重跑阶段4(改代码后,复用已有 merged.profdata)
```

五个子任务:`pgo-clean` / `pgo-instrument` / `pgo-run` / `pgo-merge` / `pgo-build`,可单独执行。

## 关键说明

1. **负载质量决定收益**。默认负载 = `cargo test --release`(smoke 测试覆盖 LSP duplex 会话与分析热路径)。
   最佳负载是手动跑一次真实 LSP 会话:打开 godot-test 工程、连续编辑、hover/completion/references 各触发若干次——多份 profraw 会在 merge 时一起合并,追加即可。
2. **profile-use 构建请勿被覆盖**:阶段 4 的产物就是发布产物。之后若用普通 `cargo build --release`(无 RUSTFLAGS)会重新编译并覆盖掉 PGO 优化结果。
3. **RUSTFLAGS 变更触发全量重编**,插桩构建与优化构建各清一次缓存,属正常现象。
4. 参数覆盖:`cargo make --env PGO_DIR=<dir> pgo`(profile 数据目录);`LLVM_BIN=<dir>` 仅在 llvm-profdata 不在 PATH 时需要(OS 环境变量或 --env 均可)。本文件不含任何机器特定路径,可直接随仓库分发。

## 实测基线(2026-10-16,本机)

- 工具链:rustc + scoop LLVM 23.1.3(`llvm-profdata.exe`)+ cargo-make 0.37.24
- 验证方式:独立 demo crate 走完整 `cargo make pgo`(clean → instrument → run → merge → build),产物运行正确;
  负路径(空目录 merge)正确报错。
- 踩坑记录:cargo-make 0.37 Windows 下 `script_runner = "@shell"` 退化为 cmd、`"@powershell"` 取不到退出码——
  两个脚本任务均改用 `command = "powershell"` + `args` 直接 spawn,稳定。

## 后续:BOLT 若可用

条件触发(见 rejected-designs.md):上游支持 PE/COFF,或本机有 WSL2 + 需要分发 Linux 二进制——届时
BOLT 工作流为 `--emit-relocs` 链接 → perf/perf2bolt → llvm-bolt -update-code-layout → 重链接,可在 WSL2 内补一档 cargo-make 任务。
