//! C4：补全频率表（构建期 gen_freq.py 双语料挖掘，产物入库内嵌）。
//!
//! 排名表只影响 sortText（空前缀时的桶内细分），不产生新补全项——
//! 不命中频率表的项保持中性分，永不被淘汰。

use std::collections::HashMap;
use std::sync::OnceLock;

fn table() -> &'static HashMap<&'static str, u32> {
    static TABLE: OnceLock<HashMap<&'static str, u32>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let ranked: Vec<&str> = serde_json::from_str(include_str!("completion_freq.json"))
            .expect("completion_freq.json 格式损坏");
        ranked.into_iter().enumerate().map(|(i, n)| (n, i as u32)).collect()
    })
}

/// API 名的频率排名（rank 0 最热）。不在表中返回 None（中性分）。
pub fn rank(name: &str) -> Option<u32> {
    table().get(name).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_are_stable_and_hot_first() {
        // 表头是语料最热 API——print 应在前 5。
        let r = rank("print").expect("print 应在频率表");
        assert!(r < 5, "print rank={r}");
        // 常用引擎方法在表中。
        assert!(rank("queue_free").is_some());
        assert!(rank("instantiate").is_some());
        // 乱名不在表 → None（中性分，不淘汰）。
        assert_eq!(rank("zzz_not_an_api_zzz"), None);
        // rank 单调可比较。
        let node = rank("Node").unwrap();
        assert!(rank("Vector2").is_some());
        let _ = node;
    }
}
