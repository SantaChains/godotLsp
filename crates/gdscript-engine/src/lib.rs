//! `gdscript-engine` —— Godot 引擎 API 模型摄取层（R2 实现骨架）。
//!
//! 设计见 `docs/from-scratch-design.md` §7。本 crate 在 R2 负责：
//! 1. 摄取 Godot 官方 `extension_api.json` 与 `doc/classes/*.xml`；
//! 2. 归一化为内部 `EngineApi`（类/方法/属性/信号/枚举/常量 + 文档）；
//! 3. 构建期 codegen 为零拷贝二进制，运行期 `from_bytes` 加载。
//!
//! R0 阶段仅为占位骨架，保证 workspace 可构建。

/// 引擎模型顶层结构（R2 充实）。
#[derive(Debug, Default, Clone)]
pub struct EngineApi {
    /// 暂存 Godot 版本标识，R2 由 `extension_api.json` 的 `header.version` 填充。
    pub version: String,
}

impl EngineApi {
    /// 占位构造；R2 改为从字节加载。
    pub fn empty() -> Self {
        Self::default()
    }

    /// 占位；R2 实现零拷贝加载。
    pub fn from_bytes(_bytes: &[u8]) -> Option<Self> {
        None
    }
}
