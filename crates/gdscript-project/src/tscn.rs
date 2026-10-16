//! tscn/tres 文本资源解析（P2.7）。
//!
//! 规格：docs/engine-analyzer.md N 章（resource_format_text.cpp）。
//! 行级状态机，**宽容解析**：畸形行跳过、四字段不全的 connection 照记录
//! （引擎行为是拒载，但 LSP 只做导航不做验收，宁可给链接也不静默丢）。
//!
//! 覆盖节头：
//! - `[gd_scene load_steps=N format=3 uid="uid://..."]`
//! - `[gd_resource type="..." script_class="..." format=3 uid="uid://..."]`
//! - `[ext_resource type="Script" path="res://x.gd" uid="uid://..." id="1_abc"]`
//! - `[sub_resource type="Curve" id="Curve_x"]`
//! - `[node name="X" type="Y" parent="P" index="0" instance=ExtResource("1")]`
//! - `[connection signal="s" from="a" to="b" method="m" binds="[...]"]`
//! - `[editable path="..."]`

/// 一个解析出的场景/资源文件。
#[derive(Debug, Default, Clone)]
pub struct SceneFile {
    /// res:// 相对路径。
    pub path: String,
    /// 头部自嵌 uid（`uid://` 前缀已剥）。
    pub uid: Option<String>,
    /// `[gd_scene format=3]` 或 `[gd_resource ...]`。
    pub kind: SceneKind,
    pub ext_resources: Vec<ExtResource>,
    pub sub_resources: Vec<SubResource>,
    pub nodes: Vec<NodeDecl>,
    pub connections: Vec<Connection>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SceneKind {
    #[default]
    Unknown,
    Scene,
    Resource,
}

#[derive(Debug, Clone)]
pub struct ExtResource {
    pub type_: String,
    /// res:// 路径（原样，含 res:// 前缀）。
    pub path: String,
    /// 自嵌 uid（`uid://` 前缀已剥）。
    pub uid: Option<String>,
    /// 场景内引用 id（如 "1_abc"）。
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct SubResource {
    pub type_: String,
    pub id: String,
}

#[derive(Debug, Default, Clone)]
pub struct NodeDecl {
    pub name: String,
    /// 内置类名或 class_name；None = 继承 parent 实例类型。
    pub type_: Option<String>,
    /// tscn parent 路径原样（"." = 根直接子级；None = 根）。
    pub parent: Option<String>,
    /// instance=ExtResource("id") 的 id。
    pub instance: Option<String>,
    pub unique_name_in_owner: bool,
    /// 文件内行号（0 起，节头行）。
    pub line: usize,
}

/// `[connection]` 四字段。`complete=false` 表示缺字段——引擎 🔴 拒载，
/// 保留给诊断层做提示（P2.10 CodeLens 只消费 complete 的）。
#[derive(Debug, Clone)]
pub struct Connection {
    pub signal: String,
    pub from: String,
    pub to: String,
    pub method: String,
    pub complete: bool,
    pub line: usize,
}

impl SceneFile {
    /// 本文件自嵌 uid → 自身 path 的映射项（供全局 uid 索引合并）。
    pub fn uid_entry(&self) -> Option<(String, String)> {
        self.uid.as_ref().map(|u| (u.clone(), self.path.clone()))
    }

    /// instance id → ext_resource 的 res:// 路径。
    pub fn ext_path(&self, id: &str) -> Option<&str> {
        self.ext_resources.iter().find(|e| e.id == id).map(|e| e.path.as_str())
    }

    /// 场景根节点名（第一个无 parent 的 [node]）。
    pub fn root_name(&self) -> Option<&str> {
        self.nodes.iter().find(|n| n.parent.is_none()).map(|n| n.name.as_str())
    }

    /// 按 tscn parent 路径语义（N2：前置 "." 变相对根）列出某节点的直接子节点名。
    pub fn children_of(&self, parent_path: &str) -> Vec<&NodeDecl> {
        self.nodes
            .iter()
            .filter(|n| n.parent.as_deref() == Some(parent_path))
            .collect()
    }
}

/// 解析 tscn/tres 文本。`path` 仅用于回填 `SceneFile::path`。
pub fn parse(path: &str, text: &str) -> SceneFile {
    let mut f = SceneFile { path: path.to_string(), ..Default::default() };
    // 当前 [node] 段的下标（属性行归属用）；None = 不在 node 段。
    let mut cur_node: Option<usize> = None;

    for (ln, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with(';') {
            continue;
        }
        if line.starts_with('[') {
            // 节头：取到匹配 ']' 前的节名与属性串。
            let inner = line.trim_start_matches('[');
            let inner = match inner.find(']') {
                Some(e) => &inner[..e],
                None => inner, // 未闭合仍宽容解析
            };
            let (head, attrs) = match inner.split_once(char::is_whitespace) {
                Some((h, a)) => (h, a),
                None => (inner, ""),
            };
            match head {
                "gd_scene" => {
                    f.kind = SceneKind::Scene;
                    f.uid = attr_uid(attrs);
                    cur_node = None;
                }
                "gd_resource" => {
                    f.kind = SceneKind::Resource;
                    f.uid = attr_uid(attrs);
                    cur_node = None;
                }
                "ext_resource" => {
                    // 引擎四字段缺一即拒载；LSP 宽容：type/id 缺失才丢（引用不了）。
                    let type_ = attr(attrs, "type").unwrap_or_default();
                    let id = attr(attrs, "id").unwrap_or_default();
                    if !id.is_empty() {
                        f.ext_resources.push(ExtResource {
                            type_,
                            path: attr(attrs, "path").unwrap_or_default(),
                            uid: attr_uid(attrs),
                            id,
                        });
                    }
                    cur_node = None;
                }
                "sub_resource" => {
                    if let Some(id) = attr(attrs, "id") {
                        f.sub_resources.push(SubResource {
                            type_: attr(attrs, "type").unwrap_or_default(),
                            id,
                        });
                    }
                    cur_node = None;
                }
                "node" => {
                    let name = attr(attrs, "name").unwrap_or_default();
                    // 无 name 的 [node type="X"] 是根（编辑器不写 name 时引擎取 type）。
                    let name = if name.is_empty() { attr(attrs, "type").unwrap_or_default() } else { name };
                    let idx = f.nodes.len();
                    // 节头内联 unique_name_in_owner=true（编辑器写独立属性行，宽容兼收）。
                    let uniq_inline = attr(attrs, "unique_name_in_owner")
                        .is_some_and(|v| v == "true");
                    f.nodes.push(NodeDecl {
                        name,
                        type_: attr(attrs, "type"),
                        parent: attr(attrs, "parent"),
                        instance: ext_ref(attrs, "instance"),
                        unique_name_in_owner: uniq_inline,
                        line: ln,
                    });
                    cur_node = Some(idx);
                }
                "connection" => {
                    let signal = attr(attrs, "signal");
                    let from = attr(attrs, "from");
                    let to = attr(attrs, "to");
                    let method = attr(attrs, "method");
                    let complete =
                        signal.is_some() && from.is_some() && to.is_some() && method.is_some();
                    f.connections.push(Connection {
                        signal: signal.unwrap_or_default(),
                        from: from.unwrap_or_default(),
                        to: to.unwrap_or_default(),
                        method: method.unwrap_or_default(),
                        complete,
                        line: ln,
                    });
                    cur_node = None;
                }
                _ => {
                    // editable / 节属性区里的未知节头：退出 node 属性上下文。
                    cur_node = None;
                }
            }
        } else if let Some(idx) = cur_node {
            // node 段属性行。
            if let Some((k, v)) = line.split_once('=')
                && k.trim() == "unique_name_in_owner" && v.trim() == "true" {
                    f.nodes[idx].unique_name_in_owner = true;
                }
        }
    }
    f
}

/// attr + uid 前缀剥离（SceneFile 存储约定：uid 不含 `uid://` 前缀）。
fn attr_uid(attrs: &str) -> Option<String> {
    attr(attrs, "uid").map(|u| u.trim_start_matches("uid://").to_string())
}

/// 从 `k="v" k2="v2"` 或 `k=v` 属性串提取字符串值。
fn attr(attrs: &str, key: &str) -> Option<String> {
    let pat = format!("{}=", key);
    let mut rest = attrs;
    while let Some(p) = rest.find(&pat) {
        // 边界检查：前面不是标识符字符（避免 unique_name 匹配 name）。
        let boundary_ok = p == 0
            || !rest[..p]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_');
        let after = &rest[p + pat.len()..];
        let after = after.trim_start();
        if boundary_ok && !after.is_empty() {
            if let Some(v) = quoted(after) {
                return Some(v);
            }
            // 宽容：无引号裸值（如 unique_name_in_owner=true），取到空白为止。
            if after.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
                let word: String =
                    after.chars().take_while(|&c| !c.is_whitespace() && c != ',').collect();
                return Some(word);
            }
        }
        rest = &rest[p + pat.len().max(1)..];
    }
    None
}

/// `after` 以 `"..."` 开头 → 提取并反转义（c_escape_multiline：只转义 \\ 与 "）。
fn quoted(after: &str) -> Option<String> {
    let mut ch = after.chars();
    if ch.next() != Some('"') {
        return None;
    }
    let mut out = String::new();
    let mut esc = false;
    for c in ch {
        if esc {
            match c {
                'n' => out.push('\n'),
                't' => out.push('\t'),
                'r' => out.push('\r'),
                other => out.push(other), // \\ 和 \" 都落到这里
            }
            esc = false;
        } else {
            match c {
                '\\' => esc = true,
                '"' => return Some(out),
                other => out.push(other),
            }
        }
    }
    None // 未闭合
}

/// `instance=ExtResource("1")` → Some("1")。兼容 SubResource 形式返回 None。
fn ext_ref(attrs: &str, key: &str) -> Option<String> {
    let v = attr_raw_call(attrs, key)?;
    let v = v.trim();
    let id = v.strip_prefix("ExtResource(")?.strip_suffix(')')?;
    Some(id.trim().trim_matches('"').to_string())
}

/// 提取 `key=...` 的右侧到下一个键或行尾（供 ext_ref 解析函数调用形式）。
fn attr_raw_call(attrs: &str, key: &str) -> Option<String> {
    let pat = format!("{}=", key);
    let p = attrs.find(&pat)?;
    let after = attrs[p + pat.len()..].trim_start();
    // 取到下一个 ` key=` 或行尾。
    let end = after
        .match_indices(&pat)
        .find(|(i, _)| {
            let i = *i;
            i == 0 || !after[..i].chars().next_back().is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
        })
        .map(|(i, _)| i)
        .unwrap_or(after.len());
    Some(after[..end].trim().to_string())
}

/// uid 头部快扫：只读前几行提 `[gd_scene/gd_resource ... uid="uid://..."]`。
/// 供全盘扫描用，比完整 parse 便宜（不碰属性行）。
pub fn scan_header_uid(text: &str) -> Option<String> {
    for line in text.lines().take(8) {
        let t = line.trim();
        if !t.starts_with("[gd_scene") && !t.starts_with("[gd_resource") {
            if t.starts_with('[') {
                return None; // 头不是第一行 → 畸形，放弃
            }
            continue;
        }
        return attr_uid(t.trim_start_matches('[').trim_end_matches(']'));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"[gd_scene load_steps=4 format=3 uid="uid://abc123"]

[ext_resource type="Script" path="res://player.gd" uid="uid://scr1" id="1_p"]
[ext_resource type="Texture2D" path="res://icon.svg" id="2_i"]

[sub_resource type="Curve" id="Curve_1"]

[node name="Player" type="CharacterBody2D"]
script = ExtResource("1_p")

[node name="Sprite" type="Sprite2D" parent="."]
texture = ExtResource("2_i")

[node name="HP" type="Label" parent="Sprite"]
unique_name_in_owner = true

[connection signal="pressed" from="Sprite/HP" to="." method="_on_hp_pressed"]

[node name="Tail" type="Node2D" parent="."]
"#;

    #[test]
    fn parses_all_sections() {
        let f = parse("res://player.tscn", SAMPLE);
        assert_eq!(f.kind, SceneKind::Scene);
        assert_eq!(f.uid.as_deref(), Some("abc123"));
        assert_eq!(f.ext_resources.len(), 2);
        assert_eq!(f.ext_resources[0].id, "1_p");
        assert_eq!(f.ext_resources[0].uid.as_deref(), Some("scr1"));
        assert_eq!(f.sub_resources.len(), 1);
        assert_eq!(f.nodes.len(), 4);
        assert_eq!(f.root_name(), Some("Player"));
        // instance 引用
        assert_eq!(f.nodes[0].instance, None);
        // unique_name 归属正确节点（HP 不是 Player）
        assert!(f.nodes[2].unique_name_in_owner);
        assert!(!f.nodes[0].unique_name_in_owner);
        // 连接完整
        assert!(f.connections[0].complete);
        assert_eq!(f.connections[0].from, "Sprite/HP");
        assert_eq!(f.connections[0].method, "_on_hp_pressed");
    }

    #[test]
    fn uid_entry_and_ext_path() {
        let f = parse("res://player.tscn", SAMPLE);
        assert_eq!(f.uid_entry(), Some(("abc123".into(), "res://player.tscn".into())));
        assert_eq!(f.ext_path("1_p"), Some("res://player.gd"));
        assert_eq!(f.ext_path("nope"), None);
    }

    #[test]
    fn incomplete_connection_recorded() {
        let t = "[connection signal=\"hit\" from=\".\"]\n";
        let f = parse("res://x.tscn", t);
        assert_eq!(f.connections.len(), 1);
        assert!(!f.connections[0].complete);
    }

    #[test]
    fn attr_boundary_no_prefix_hijack() {
        // unique_name_in_owner 不得劫持 name= 查找
        let t = "[node name=\"A\" parent=\".\" unique_name_in_owner=true]\n";
        let f = parse("res://x.tscn", t);
        assert_eq!(f.nodes[0].name, "A");
        assert!(f.nodes[0].unique_name_in_owner);
    }

    #[test]
    fn escaped_quotes_in_values() {
        let t = "[node name=\"Say \\\"hi\\\"\" type=\"Label\"]\n";
        let f = parse("res://x.tscn", t);
        assert_eq!(f.nodes[0].name, "Say \"hi\"");
    }

    #[test]
    fn malformed_lines_skipped() {
        let t = "garbage\n[node name=\"A\"]\nbroken = = =\n[connection]\n";
        let f = parse("res://x.tscn", t);
        assert_eq!(f.nodes.len(), 1);
        assert!(!f.connections[0].complete);
    }

    #[test]
    fn resource_kind_with_uid() {
        let t = "[gd_resource type=\"Resource\" script_class=\"Foo\" format=3 uid=\"uid://r9\"]\n\n[resource]\n";
        let f = parse("res://x.tres", t);
        assert_eq!(f.kind, SceneKind::Resource);
        assert_eq!(f.uid.as_deref(), Some("r9"));
    }

    #[test]
    fn header_scan_only_reads_head() {
        assert_eq!(scan_header_uid(SAMPLE).as_deref(), Some("abc123"));
        let t = "[gd_resource type=\"X\" format=3 uid=\"uid://q\"]\n";
        assert_eq!(scan_header_uid(t).as_deref(), Some("q"));
        assert_eq!(scan_header_uid("no header"), None);
    }
}
