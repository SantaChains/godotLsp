# 图形学速览

**Godot 渲染管线：剔除 → 阴影 → 不透明 → 透明排序 → 后处理。理解每阶段为什么这样排序。**

## 五阶段

| 阶段 | 做什么 | 为什么这样 |
|---|---|---|
| 剔除 | 视锥体外、背面的物体不提交 | 省 GPU |
| 阴影 | 从光源视角渲染深度贴图 | 生成阴影用 |
| 不透明 | 从前到后渲染 | early-Z 快速丢弃被遮挡片元 |
| 透明 | 从后到前渲染 | 混合顺序影响结果，必须远→近 |
| 后处理 | 辉光、DOF、颜色校正 | 整屏处理 |

> 值得注意的是：**不透明从前到后，透明从后到前**，顺序相反。不透明靠深度缓冲 early-Z 省片元，透明靠混合必须严格按距离排序。

## 光照模型：Blinn-Phong 与 PBR

教学上先用 Blinn-Phong 理解三个向量：法线 N、光源方向 L、视线方向 V。

```
漫反射 = max(0, N·L)
高光   = (N·半程向量 H) 的幂（H 归一化(L+V)）
```

> 需要注意：**Godot 的 StandardMaterial3D 默认不是 Blinn-Phong，而是基于物理的 PBR**（GGX 高光 / Smith 几何遮蔽 / Schlick 菲涅尔 / Disney 漫反射）。Blinn-Phong 只是理解"漫反射靠 N·L、高光靠半程向量"的简化模型；Godot 的 2D 调制、自定义 canvas_item shader 里你可以自己写 Blinn-Phong 式经验光照，3D 标准材质走的是 PBR 管线。

## 后处理概念

| 技术 | 效果 | 开销 |
|---|---|---|
| 法线贴图 | 假凹凸，不加几何面数 | 低 |
| 阴影映射 | 深度贴图生成软阴影 | 中 |
| 辉光 Bloom | 高亮区域溢出光晕 | 中高 |
| SDFGI | 有向距离场全局光照 | 高 |
| SSAO | 屏幕空间环境光遮蔽 | 中 |

## Godot Shader 入口

```gdshader
// 2D 着色器
shader_type canvas_item;

void fragment() {
    // TEXTURE = 原图，UV = 当前像素坐标
    COLOR = texture(TEXTURE, UV);
    // 加一个红色调
    COLOR.r += 0.2;
}
```

```gdshader
// 3D 着色器
shader_type spatial;

void fragment() {
    ALBEDO = vec3(0.8, 0.2, 0.2);  // 红色漫反射
}
```

| shader_type | 用途 |
|---|---|
| `canvas_item` | 2D 精灵、UI |
| `spatial` | 3D 模型 |
| `particles` | 粒子 |

## 延伸阅读

- 本站：[物理与渲染预算](../perf/04-physics-render.md)
- 官方：[Shaders](https://docs.godotengine.org/en/stable/tutorials/shaders/index.html)
