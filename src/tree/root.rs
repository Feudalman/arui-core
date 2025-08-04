//! 项目树根节点
//! 用于初始化操作和启动目录树分析
use crate::tree::config::ProjectConfig;
use crate::tree::node::TreeNode;
use crate::utils::{check_path, generate_id};
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

/// 项目目录树根节点
/// 用于初始化操作和启动目录树分析
pub struct ProjectTree {
    pub id: String,                    // 项目id
    pub name: String,                  // 别名
    pub path: String,                  // 根路径
    pub root: Option<TreeNode>,        // 根节点
    pub config: Option<ProjectConfig>, // 配置
}

/// 初始化项目及构建属性
impl ProjectTree {
    /// 通过一系列参数初始化项目树
    /// - name：项目别名
    /// - path：根路径
    /// - config：配置
    ///
    /// 但是需要注意，该操作仅初始化一个基础的项目树对象，可以通过该对象启动项目树的构建和分析：
    /// - build： 构建项目树，生成各个节点，但不包含总结信息
    /// - summarize： 遍历已有项目树，获取各个节点的总结信息
    ///
    /// # Example
    ///
    /// ```rust
    /// use arui_core::tree::root::ProjectTree;
    /// const NAME: &str = "test";
    /// const PATH: &str = ".";
    /// // 需要使用可变类型来初始化
    /// let mut project = ProjectTree::new(NAME, PATH, None);
    /// // 需要通过 build 和 summary 方法来启动项目树的构建和分析
    /// project.build().unwrap();
    /// project.summarize().unwrap();
    /// ```
    pub fn new<S, I>(name: S, path: I, config: Option<ProjectConfig>) -> Self
    where
        S: Into<String>,
        I: Into<String>,
    {
        ProjectTree {
            id: generate_id(),
            name: name.into(),
            path: path.into(),
            root: None,
            config,
        }
    }

    /// 动态检查节点路径是否合法
    ///
    /// # Example
    ///
    /// ```rust
    /// use arui_core::tree::root::ProjectTree;
    /// // 只有在路径合法的情况下，才能进行项目树构建，但可正常初始化项目
    /// let can_build = ProjectTree::new("test", "/@not_exist", None).is_valid();
    /// println!("{}", can_build); // should be false
    /// ```
    pub fn is_valid(&self) -> bool {
        check_path(&self.path).is_ok()
    }

    /// "种植"一棵项目树
    /// - name：项目别名
    /// - path：根路径
    /// - config：配置
    ///
    /// 该操作和 `new` 的不同的地方在于，`new` 方法仅初始化项目树入口，并不完成后续操作：
    /// - build： 从 `path` 启动，遍历并生成项目树
    /// - summary： 从 `root` 启动，遍历并生成项目树各节点的总结信息
    ///
    /// # Example
    ///
    /// ```rust
    /// use arui_core::tree::root::ProjectTree;
    /// const NAME: &str = "test";
    /// const PATH: &str = ".";
    /// // 如果只用 `plant`，后续也不需要更新项目树，则可不使用 `mut`
    /// let project = ProjectTree::plant(NAME, PATH, None);
    /// ```
    pub fn plant<S, I>(name: S, path: I, config: Option<ProjectConfig>) -> Self
    where
        S: Into<String>,
        I: Into<String>,
    {
        let mut tree = ProjectTree::new(name, path, config);
        tree.build().expect("project build panic");
        tree.summarize().expect("project summarize panic");
        tree
    }

    // ------------------------- 遍历构建节点 -------------------------

    /// 构建项目文件树（不包含summary信息）
    /// 通过 `path` 启动，遍历并生成项目树，但仅初始化各级树结构：
    /// - path：节点对应文件/目录的路径
    /// - is_dir：是否是文件夹
    /// - children：子节点（is_dir为true时有值）
    /// 其中并不包含 `summary` 字段的获取，需要单独调用 `summarize` 方法来获取
    ///
    /// # Example
    ///
    /// ```rust
    /// use arui_core::tree::root::ProjectTree;
    /// let mut project = ProjectTree::new("test", "./src", None);
    /// project.build().unwrap();
    /// ```
    pub fn build(&mut self) -> Result<()> {
        // 如果路径不合法，返回错误
        // TODO: 修改为自定义错误
        if !self.is_valid() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid path",
            ));
        }
        let root_path = PathBuf::from(&self.path);
        // 尝试遍历构建项目树，生成各个节点
        self.root = Some(Self::build_tree_node(&root_path)?);
        Ok(())
    }

    /// 递归构建树节点
    fn build_tree_node(path: &Path) -> Result<TreeNode> {
        // 获取文件元数据
        // TODO: 自定义错误处理
        let metadata = fs::metadata(path)?;
        let is_dir = metadata.is_dir();
        // 创建节点
        let mut node = TreeNode::new(path.to_string_lossy().into_owned(), is_dir);
        // 如果是目录，递归构建该节点的子节点
        // TODO：自定义错误
        if is_dir {
            let mut children = Vec::new();
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                children.push(Self::build_tree_node(&entry.path())?);
            }
            node.children = Some(children);
        }

        Ok(node)
    }

    // ------------------------- 生成总结信息 -------------------------

    pub fn summarize(&mut self) -> Result<()> {
        // 如果根节点不存在，返回错误
        if self.root.is_none() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "root is none, please build by `build()` first",
            ));
        }
        // 如果根路径不合法，返回错误
        if !self.is_valid() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid root path, please check it",
            ));
        }
        // 递归获取总结信息
        self.root.as_mut().unwrap().upsert_summary();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::visible::ProjectTreeVisible;

    #[test]
    fn test_new() {
        let name = "test";
        let path = ".".to_string();
        let tree = ProjectTree::new(name, path, None);
        assert_eq!(tree.id.len(), 36);
        assert_eq!(tree.name, name.to_string());
        assert_eq!(tree.path, ".".to_string());
    }

    #[test]
    fn test_get_valid() {
        let valid_tree = ProjectTree::new("test".to_string(), "./src".to_string(), None);
        assert_eq!(valid_tree.is_valid(), true);
        let invalid_tree = ProjectTree::new("test".to_string(), "/not_exist".to_string(), None);
        assert_eq!(invalid_tree.is_valid(), false);
    }

    #[test]
    fn test_build_project_tree() {
        let name = "test";
        let path = "./src";
        let mut tree = ProjectTree::new(name, path, None);
        tree.build().expect("panic");
        assert_eq!(tree.root.is_some(), true);
        assert_eq!(tree.name, "test".to_string());
        assert_eq!(tree.path, "./src".to_string());
        // 打印一下看看结构是否正确
        tree.print_tree();
    }

    #[test]
    fn test_plant() {
        let name = "test";
        let path = "./src";
        let tree = ProjectTree::plant(name, path, None);
        assert_eq!(tree.root.is_some(), true);
        assert_eq!(tree.name, "test".to_string());
        assert_eq!(tree.path, "./src".to_string());
        // 打印一下看看结构是否正确
        tree.print_tree();
        println!("{}", tree.root.unwrap());
    }
}
