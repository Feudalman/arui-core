# ARUI-CORE

一个简单的库，用于分析指定目录下的文件树。

> 该项目由个人不定期维护，且可以预期地将长期处于开发中。

## 目标 Features

- [x] 生成指定目录下的文件树结构
- [x] 遍历文件树生成各节点的总结信息
- [ ] 自定义错误类型
- [ ] 提供配置字段，用于控制文件树生成的行为
- [ ] 提供更多总结信息的获取
- [ ] 增加/优化项目树可视化 API

## 用法

该项目作为 `lib`，以依赖形式导入您的项目，通过几个关键数据结构提供主要功能。

### Build and Summarize

这里多步构建完整项目树，通过 `new`、`build`、`summarize` 可以选择合适节点完成对应操作：

- `new`：实例化项目树对象
- `build`：通过初始化 `path` 启动遍历，获取项目树结构
- `summarize`：通过构建后的项目树自底向上分析各个节点的总结信息

> 注意！`build`/`summarize`操作为原地操作，即直接修改项目树实例，若分步则需提前声明可变变量。

```rust
use arui_core::ProjectTree;

let name = "test";
let path = "./tests/examples";
// init with name/path/config
let mut tree = ProjectTree::new(name, path, None);
// build to get project file tree
tree.build().expect("build failed");
// summarize to get summaries of tree nodes
tree.summarize().expect("summarize failed");
```

### Plant

直接“种植”一棵项目树，即不分步，直接完成所有操作。
若仅种植一次，后续不再更新项目树信息，则可不声明为可变变量。

```rust
use arui_core::ProjectTree;

let name = "test";
let path = "./tests";
let tree = ProjectTree::plant(name, path, None);
```

### Show

将项目树基础信息输出到终端。

```rust
use arui_core::ProjectTree;

let tree = ProjectTree::plant("test", "./tests", None);

tree.show()
```

### Print_tree

遍历项目树，按照构建结果逐条输出节点基础信息到终端。

```rust
use arui_core::ProjectTree;

let tree = ProjectTree::plant("test", "./tests", None);

tree.print_tree()
```
