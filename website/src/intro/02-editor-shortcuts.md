# 编辑器高效快捷键

**以下为官方默认键位（Windows/Linux）；macOS 把 Ctrl 换成 Cmd。** 键位随上下文生效，同一组合在不同面板动作不同。自定义入口：Editor > Editor Settings > Shortcuts，按 `editor/...` 动作名检索。

## 播放与全局

| 动作 | 键 | 说明 |
|---|---|---|
| Play 运行主场景 | F5 | 从 Project Settings 指定的主场景启动 |
| Play Scene 运行当前场景 | F6 | 只跑当前打开的场景，调试单场景用 |
| Pause | F7 | 暂停运行中场景 |
| Play Custom Scene | Ctrl + Shift + F5 | 选择任意场景启动 |
| Save Scene | Ctrl + S | 保存当前场景 |
| Save All | Ctrl + Shift + Alt + S | 保存全部打开的场景与脚本 |
| Undo / Redo | Ctrl + Z / Ctrl + Shift + Z | 编辑器级撤销（脚本编辑器重做是 Ctrl + Y） |
| 退出到项目列表 | Ctrl + Shift + Q | 不退出编辑器，回工程选择器 |
| 截图 | Ctrl + F12 | 抓当前视口 |
| 全屏切换 | Shift + F11 | |

> F5 与 F6 的区别：改完只想验证当前节点逻辑用 F6，避免主场景加载整条流程；发布前回归用 F5。

## 场景树 dock

| 动作 | 键 |
|---|---|
| Add Child Node（加子节点） | Ctrl + A |
| Rename（F2 内联改名） | F2 |
| Batch Rename | Ctrl + F2 |
| Duplicate 复制节点 | Ctrl + D |
| Delete | Del |
| Force Delete（无确认） | Shift + Del |
| Copy Node Path | Ctrl + Shift + C |
| 同级上移 / 下移 | Ctrl + ↑ / Ctrl + ↓ |

## 2D 画布

| 动作 | 键 |
|---|---|
| 选择 Select | Q |
| 移动 Move | W |
| 旋转 Rotate | E |
| 缩放 Scale | S |
| 标尺 Ruler | R |
| 平移视图 | Space + 拖 |
| 放大 / 缩小 / 重置缩放 | Ctrl + = / Ctrl + - / Ctrl + 0 |
| 智能吸附 | Shift + S |
| 网格吸附 | Shift + G |
| 预览画布缩放 | Ctrl + Shift + P |

> 4.6 起选择与变换解耦：Select-only 只点选不出现 gizmo，Transform 模式才显示移动/旋转/缩放手柄，避免误拖。

## 3D 视口

| 动作 | 键 |
|---|---|
| Freelook 自由飞行切换 | Shift + F |
| 旋转工具 | E |
| 右视图 / 左视图 | Num3 / Alt + Num3 |
| 透视/正交切换 | Num5 |
| 插关键帧 | K |
| 聚焦原点 | O |
| 对齐变换到视图 | Ctrl + Alt + M |
| 对齐旋转到视图 | Ctrl + Alt + F |
| 单/双/四视口 | Ctrl + 1 / Ctrl + 2 / Ctrl + 4 |
| 双击 F 跟随选中（4.7 两次按下可跟随移动） | F |
| Trackball 轨迹球旋转（4.7） | U |
| Vertex Snap（4.7） | B |

## 脚本编辑器

| 动作 | 键 |
|---|---|
| 保存脚本 | Ctrl + S | 与场景保存同键，焦点在脚本编辑器时存脚本 |
| Save All | Ctrl + Shift + Alt + S | 保存全部打开的场景与脚本 |
| Soft Reload Script（热重载，不重启运行） | Ctrl + Shift + R |
| 关闭当前文件 | Ctrl + W |
| Run File（独立运行脚本） | Ctrl + Shift + X |
| Quick Open Script | Ctrl + Alt + O |
| 编辑区放大/缩小/重置 | Ctrl + = / Ctrl + - / Ctrl + 0 |

## 文件系统 dock

| 动作 | 键 |
|---|---|
| Copy Path | Ctrl + C |
| Duplicate | Ctrl + D |
| Delete | Del |

## 提速习惯

- 全程键盘加节点：Ctrl + A 直接搜类名，比鼠标点菜单快。
- F2 内联改名配合 Ctrl + D 复制变体，节点命名一次到位，避免后面重接信号。
- 调试单场景用 F6，跳过主场景初始化。
- 高频动作（如切回 Game 面板、打开某 dock）在 Shortcuts 里自行绑键，官方未给默认。

## 延伸阅读

- 官方：[Default editor shortcuts](https://docs.godotengine.org/en/stable/tutorials/editor/default_key_mapping.html)
