//! 解析错误类型。

/// 单个解析诊断。保证「有界、非 panic」——解析器永远记录错误而非崩溃。
#[derive(Clone, Debug)]
pub struct ParseError {
    pub message: String,
    /// 错误发生的字节区间（尽可能精确；无法定位时回退到 token 起止）。
    pub start: usize,
    pub end: usize,
    pub line: u32,
}

impl ParseError {
    pub fn new(message: impl Into<String>, start: usize, end: usize, line: u32) -> Self {
        ParseError { message: message.into(), start, end, line }
    }
}
