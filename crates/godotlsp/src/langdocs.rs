//! 语言级关键字/操作符文档（hover 兜底层）。
//!
//! 这些符号不在 godot-docs 类参考语料中（非类成员），是**语料正交层**：
//! - 描述：来自官方 GDScript reference 的手工整理快照（恢复自旧架构 gdscript_docs.rs，
//!   is/as/in/and/or/not/await 采用中文详述版）；hover 渲染附语法篇站内链接。
//! - 链接目标：`doclinks::syntax_url`，与网站语法篇页面一一对应。
//! - `PI`/`TAU`/`INF`/`NAN` 不在此表——它们是 @GDScript 常量，走 hoverdata 语料（`@GDScript.PI`）。

use crate::doclinks;
use crate::hoverdata;

/// (关键字, 描述, 语法篇目标 `slug#kw-名`)。
/// 锚点契约见规划文档 §五之三；锚点在页面中以 `{#kw-x}` 或 `<a id="kw-x">` 显式锁定，
/// `tools/check_lsp_contract.py` 逐条校验 mdbook 产物中锚点存在。
const KEYWORD_DOCS: &[(&str, &str, &str)] = &[
    ("var", r#"声明变量：`var hp: int = 100`；支持推断、导出（配合注解）。"#, "03-vars#kw-var"),
    ("const", r#"声明编译期常量：`const MAX = 10`，也支持表达式与数组/字典。"#, "03-vars#kw-const"),
    ("is", r#"类型判断：`a is Node` 为真当 a 是 Node 或其子类的实例，或 a 为指定内建类型（`a is int`）。"#, "04-types#kw-is"),
    ("as", r#"安全转型：`a as Node` 若 a 可转为 Node 则返回该实例，否则返回 null（不抛错）。"#, "04-types#kw-as"),
    ("if", r#"条件分支：`if cond:` … `elif cond2:` … `else:`。"#, "05-flow#kw-if"),
    ("elif", r#"条件分支的否则如果分支，等同 `else: if` 的简写。"#, "05-flow#kw-elif"),
    ("else", r#"条件分支的否则分支。"#, "05-flow#kw-else"),
    ("match", r#"模式匹配：`match value:` 按 pattern 分支，`_` 兜底。"#, "05-flow#kw-match"),
    ("when", r#"match 分支守卫：`pattern when cond:` 仅在条件为真时命中该分支。"#, "05-flow#kw-when"),
    ("while", r#"条件循环：`while cond:`，条件为真时重复执行。"#, "05-flow#kw-while"),
    ("for", r#"迭代循环：`for x in container:`，容器可为 Array/Dictionary/Range/String/节点。"#, "05-flow#kw-for"),
    ("break", r#"立即退出当前 for/while 循环。"#, "05-flow#kw-break"),
    ("continue", r#"立即跳到当前 for/while 循环的下一次迭代。"#, "05-flow#kw-continue"),
    ("in", r#"`value in container` 测试成员：String 子串 / Array 元素 / Dictionary 键 / Range 范围；也用于 `for x in container` 迭代。"#, "06-operators#kw-in"),
    ("and", r#"逻辑与（短路求值），符号形式 `&&` 等价。"#, "06-operators#kw-and"),
    ("or", r#"逻辑或（短路求值），符号形式 `||` 等价。"#, "06-operators#kw-or"),
    ("not", r#"逻辑非（一元），符号形式 `!` 等价。"#, "06-operators#kw-not"),
    ("func", r#"定义函数：`func name(args) -> Ret:`。"#, "07-functions#kw-func"),
    ("return", r#"从函数返回值；无返回值函数可裸 `return`。"#, "07-functions#kw-return"),
    ("void", r#"表示函数无返回值的类型标注：`func log_msg() -> void:`。"#, "07-functions#kw-void"),
    ("pass", r#"空语句：语法上需要语句但不想执行任何代码时使用（如空函数体）。"#, "07-functions#kw-pass"),
    ("static", r#"定义静态函数或静态成员变量，属类而非实例。"#, "07-functions#kw-static"),
    ("super", r#"调用父类方法：`super.method(args)` 或 `super(args)`（同名方法）。"#, "07-functions#kw-super"),
    ("class", r#"定义内部类（inner class）：`class Name:`，作用域限于外层脚本。"#, "08-classes#kw-class"),
    ("class_name", r#"把脚本注册为全局可访问的具名类：`class_name MyNode extends Node`。"#, "08-classes#kw-class_name"),
    ("extends", r#"声明当前类继承的基类：`extends Node2D`。"#, "08-classes#kw-extends"),
    ("self", r#"指代当前类实例；通常可省略，歧义或传引用时显式使用。"#, "08-classes#kw-self"),
    ("enum", r#"声明枚举：`enum State { IDLE, RUN }`，具名值即常量。"#, "08-classes#kw-enum"),
    ("preload", r#"编译期加载资源：`preload("res://x.tscn")`，路径必须是常量字符串。"#, "08-classes#kw-preload"),
    ("signal", r#"声明信号：`signal died(cause: String)`，`emit` 或 `emit_signal` 触发。"#, "10-signals#kw-signal"),
    ("await", r#"等待信号或协程完成：`await signal` 挂起当前函数直到信号触发；`await get_tree().create_timer(1.0).timeout` 是常用延时写法。"#, "12-await#kw-await"),
    ("yield", r#"Godot 3 的协程关键字，4.x 已移除，改用 await。"#, "12-await#kw-yield"),
    ("assert", r#"断言条件为真，失败时报错；非调试构建中被忽略。`assert(cond, "message")`。"#, "14-errors#kw-assert"),
    ("breakpoint", r#"编辑器断点辅助关键字，写死在脚本中的断点（区别于编辑器侧边栏点击断点）。"#, "14-errors#kw-breakpoint"),
];

/// 符号操作符：(符号, 描述, 锚点名 `op-<ascii>`)。
/// 锚点落在 06-operators「符号锚点索引」节（符号不能作 HTML id，用 ASCII 名）。
#[allow(dead_code)]
const OPERATOR_DOCS: &[(&str, &str, &str)] = &[
    ("+", r#"加法 / 字符串拼接（`"a" + "b"`）/ 数组拼接（`[1] + [2]`）。"#, "op-plus"),
    ("-", r#"减法 / 一元取负（`-x`）。"#, "op-minus"),
    ("*", r#"乘法 / 重复（`"ab" * 3` → `"ababab"`）。"#, "op-star"),
    ("/", r#"除法：整数相除也返回 float（`3 / 2 == 1.5`）。"#, "op-slash"),
    ("%", r#"取模（余数）。注意 `%Name` 在 `%` 后紧跟标识符时是「唯一节点」引用。"#, "op-percent"),
    ("**", r#"乘方（`2 ** 8 == 256`）。"#, "op-pow"),
    ("==", r#"相等比较（值相等；对象比引用请用 `is_same`）。"#, "op-eq"),
    ("!=", r#"不等比较。"#, "op-ne"),
    ("=", r#"赋值。复合赋值：`+=` `-=` `*=` `/=` `%=` `**=`。"#, "op-assign"),
    ("&&", r#"逻辑与（短路求值），关键字形式 `and` 等价。"#, "op-and"),
    ("||", r#"逻辑或（短路求值），关键字形式 `or` 等价。"#, "op-or"),
    ("!", r#"逻辑非（一元），关键字形式 `not` 等价。"#, "op-not"),
    ("&", r#"按位与。"#, "op-band"),
    ("|", r#"按位或。"#, "op-bor"),
    ("^", r#"按位异或。"#, "op-bxor"),
    ("~", r#"按位取反（一元）。"#, "op-bnot"),
    ("<<", r#"左移位（`1 << 4 == 16`）。"#, "op-shl"),
    (">>", r#"右移位。"#, "op-shr"),
    ("??", r#"空合并：`a ?? b` 当 a 为 null 时取 b，否则取 a。"#, "op-coalesce"),
    ("??=", r#"空合并赋值：当 a 为 null 时把 b 赋给 a。"#, "op-coalesce-assign"),
];

/// 关键字 hover markdown：描述 + 语法篇站内链接。非关键字返回 None。
pub fn keyword_hover(word: &str) -> Option<String> {
    let (_, desc, page) = KEYWORD_DOCS.iter().find(|(k, _, _)| *k == word)?;
    Some(format!("`{word}`\n\n{desc}\n\n[语法参考]({})", doclinks::syntax_url(page)))
}

/// @GDScript 常量兜底（PI/TAU/INF/NAN，值与描述来自语料）：非 @GDScript 常量返回 None。
pub fn gdscript_const_hover(is_real: &dyn Fn(&str) -> bool, word: &str) -> Option<String> {
    let desc = hoverdata::member("@GDScript", word)?;
    let url = doclinks::member_url("@GDScript", word, false, is_real);
    Some(format!("```gdscript\nconst {word}\n```\n{desc}\n\n[文档]({url})"))
}
