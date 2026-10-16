//! C1 补全排序骨架：分桶 relevance + fuzzy 评分 → sortText。
//!
//! 参照（lsp-design.md §6.8）：
//! - rust-analyzer：relevance 与 fuzzy 严格分离；sortText = 取反分数零填充。
//! - clangd：Quality（符号质量）× Relevance（查询相关）双维；本模块把
//!   relevance 收敛为「桶」（kind 级），fuzzy 为词面匹配分。
//!
//! sortText 编码：`{bucket:02x}{(0xFFFF-fuzzy):04x}`（6 字符，fuzzy 取反，
//! 字典序 = 优先序，客户端升序排列）。桶内按 fuzzy 细分，桶间 relevance 隔离。

/// relevance 桶（值越小越靠前）。0x00-0x0F 段预留给 C3 本文件词条面。
pub mod bucket {
    /// C3 预留：本文件标识符 / 局部变量。
    #[allow(dead_code)]
    pub const LOCAL: u8 = 0x00;
    /// 成员面（`recv.` 之后的方法/属性/常量/枚举）与注解面（`@` 独立面）。
    pub const MEMBER: u8 = 0x10;
    /// 全局函数：内建函数 + 工具函数 + 单例。
    pub const FN_GLOBAL: u8 = 0x20;
    /// 类型：引擎类 + 内建类型。
    pub const TYPE: u8 = 0x21;
    /// 全局常量 + 枚举值展开。
    pub const CONST: u8 = 0x22;
    /// 关键字。
    pub const KEYWORD: u8 = 0x23;
    /// 上下文模板（虚方法/语句/复合 snippet），C2。
    pub const SNIPPET: u8 = 0x11;
}

/// fuzzy 分数带（高 = 优）。带区间互不重叠，带内按匹配质量单调递减。
///
/// | 带 | 区间 | 含义 |
/// |---|---|---|
/// | EXACT | 0xFFFF | 完全相等 |
/// | CI_EQ | 0xFF00 | 忽略大小写相等 |
/// | PREFIX | 0xF801-0xFE00 | 大小写敏感前缀（越短越好）|
/// | CI_PREFIX | 0xF001-0xF700 | 忽略大小写前缀 |
/// | SUBSEQ | 0x8000-0xEFFF | 子序列（空隙越少越好）|
/// | NEUTRAL | 0x4000 | 空前缀中性分 |
/// | NONE | 0 | 不匹配 |
pub mod band {
    pub const EXACT: u16 = 0xFFFF;
    pub const CI_EQ: u16 = 0xFF00;
    pub const PREFIX_TOP: u16 = 0xFE00; // 带内 - extra(≤0x05FF)
    pub const CI_PREFIX_TOP: u16 = 0xF700; // 带内 - extra(≤0x06FF)
    pub const SUBSEQ: u16 = 0x8000; // + 0x6FFF - 空隙密度
    /// 空前缀中性分：桶序主导，fuzzy 不参与。
    pub const NEUTRAL: u16 = 0x4000;
    pub const NONE: u16 = 0;
}

/// C4：空前缀时按频率排名细分中性带。
///
/// rank 0（最热）→ 0x7FFF，排名越大分越低，超出 0x3FFF 名或未命中
/// → NEUTRAL（与未收录项同权垫底，不被淘汰）。带顶 0x7FFF < SUBSEQ 0x8000，
/// 带间不重叠。仅空前缀调用——有前缀时 fuzzy 分已充分区分。
pub fn neutral_by_rank(rank: u32) -> u16 {
    band::NEUTRAL + 0x3FFF - rank.min(0x3FFF) as u16
}

/// 词面匹配分：prefix 对 label。
///
/// 层级（降序）：完全相等 > 忽略大小写相等 > 前缀（越短越好）>
/// 忽略大小写前缀 > 子序列（空隙越少越好）> 不匹配。
/// 空前缀返回中性分（纯桶排序）。
pub fn fuzzy(prefix: &str, label: &str) -> u16 {
    if prefix.is_empty() {
        return band::NEUTRAL;
    }
    if label == prefix {
        return band::EXACT;
    }
    if label.starts_with(prefix) {
        let extra = (label.len() - prefix.len()) as u16;
        return band::PREFIX_TOP - extra.min(0x05FF);
    }
    let l = label.to_lowercase();
    let p = prefix.to_lowercase();
    if l == p {
        return band::CI_EQ;
    }
    if l.starts_with(&p) {
        let extra = (l.len() - p.len()) as u16;
        return band::CI_PREFIX_TOP - extra.min(0x06FF);
    }
    subsequence(&p, &l).map(|gaps| {
        band::SUBSEQ + (0x6FFFu32.saturating_sub(gaps as u32 * 0x180)) as u16
    }).unwrap_or(band::NONE)
}

/// 忽略大小写子序列匹配：返回总空隙数（跳过的 label 字符数），不匹配返回 None。
fn subsequence(prefix: &str, label: &str) -> Option<usize> {
    let mut gaps = 0usize;
    let mut it = label.bytes();
    for &pc in prefix.as_bytes() {
        loop {
            match it.next() {
                Some(lc) if lc == pc => break,
                Some(_) => gaps += 1,
                None => return None,
            }
        }
    }
    Some(gaps)
}

/// 组装 sortText：`{bucket:02x}{fuzzy:04x}`。
///
/// fuzzy 取反（`0xFFFF - score`）后编码——客户端按 sortText **升序**排，
/// 取反使高分 → 小字符串 → 排前（对齐 rust-analyzer `score ^ 0xFFFFFFFF`）。
pub fn sort_text(bucket: u8, fuzzy_score: u16) -> String {
    format!("{bucket:02x}{:04x}", 0xFFFF - fuzzy_score)
}

/// 该 fuzzy 分是否构成真实词面匹配（≥ 子序列带）。
/// 上下文模板只对「与输入有词面关系」的输入出现——空前缀不泛滥。
pub fn is_match(score: u16) -> bool {
    score >= band::SUBSEQ
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzy_tier_order() {
        let p = "col";
        let exact = fuzzy(p, "col");
        let ci_eq = fuzzy(p, "COL");
        let prefix_short = fuzzy(p, "color"); // 前缀、更短
        let prefix_long = fuzzy(p, "collision"); // 前缀、更长
        let ci_prefix = fuzzy(p, "Color");
        let subseq = fuzzy(p, "canvas_oil_lamp"); // c…o…l 子序列
        let none = fuzzy(p, "audio");

        assert!(exact > ci_eq, "完全相等 > 忽略大小写相等");
        assert!(ci_eq > prefix_short, "相等 > 前缀");
        assert!(prefix_short > prefix_long, "前缀同带内越短越好");
        assert!(prefix_long > ci_prefix, "大小写敏感前缀 > 忽略大小写前缀");
        assert!(ci_prefix > subseq, "前缀 > 子序列");
        assert!(subseq > none, "子序列 > 不匹配");
        assert_eq!(none, 0);
    }

    #[test]
    fn fuzzy_empty_prefix_is_neutral() {
        assert_eq!(fuzzy("", "anything"), band::NEUTRAL);
        // 中性分低于一切匹配带、高于不匹配——空前缀时桶序纯净主导。
        assert!(band::NEUTRAL < band::SUBSEQ);
        assert!(band::NEUTRAL > band::NONE);
    }

    #[test]
    fn subsequence_gap_density() {
        // 空隙越少分越高：紧密子序列优于稀疏子序列。
        let near = fuzzy("col", "caol");
        let far = fuzzy("col", "c_x_o_l");
        assert!(near > far, "空隙少 > 空隙多");
        assert_eq!(fuzzy("col", "xyz"), 0, "非子序列为 0");
    }

    #[test]
    fn sort_text_lexicographic() {
        // 取反编码：高分 → 小字符串（升序在前）。
        assert_eq!(sort_text(bucket::LOCAL, band::EXACT), "000000");
        assert_eq!(sort_text(bucket::LOCAL, band::NONE), "00ffff");
        // 桶主导：低桶 + 低分 < 高桶 + 高分。
        let member_low = sort_text(bucket::MEMBER, band::NONE);
        let keyword_hi = sort_text(bucket::KEYWORD, band::EXACT);
        assert!(member_low < keyword_hi);
        // 桶内 fuzzy 主导：前缀匹配（高分）排在子序列（低分）前。
        let a = sort_text(bucket::TYPE, fuzzy("col", "color"));
        let b = sort_text(bucket::TYPE, fuzzy("col", "canvas_oil_lamp"));
        assert!(a < b, "前缀 {a} 应排在子序列 {b} 之前");
    }
}
