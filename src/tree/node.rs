//! 项目树节点
//! 维护节点层级的特性，包含数据获取等等各类节点级操作。
//! 注意！项目树节点一般由 tree 代理，而不应该由用户手动控制，某些情况下可以对节点进行数据获取操作，但应该仅限于此。
pub mod count;
pub mod file;
use crate::{tree::summary::NodeSummary, utils::check_path};
use std::fmt::Display;

/// 目录树节点
/// - 节点为文件时，无子树
/// - 节点为目录时，有子树
#[derive(Debug, Clone)]
pub struct TreeNode {
    // 当前节点所处路径
    pub path: String,
    // 是否是目录
    pub is_dir: bool,
    // 如果是目录，那么遍历他的子节点
    pub children: Option<Vec<TreeNode>>,
    // 总结信息
    pub summary: NodeSummary,
}

/// 为节点实现 Display
impl Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n----- TreeNode -----\n\n- path: {}\n- is_dir: {}\n- children: {}\n- summary: {}\n\n--------------------",
            self.path,
            self.is_dir,
            self.children.as_ref().unwrap_or(&Vec::new()).len(),
            self.summary
        )
    }
}

/// 节点配置相关
impl TreeNode {
    /// 检测节点路径是否合法
    pub fn is_valid(&self) -> bool {
        check_path(&self.path).is_ok()
    }

    /// 创建一个项目树节点
    pub fn new<P>(path: P, is_dir: bool) -> Self
    where
        P: Into<String>,
    {
        TreeNode {
            is_dir,
            path: path.into(),
            children: if is_dir { Some(Vec::new()) } else { None },
            summary: NodeSummary::new(),
        }
    }
}

/// 为节点实现总结信息相关操作
impl TreeNode {
    /// 获取节点总结信息，重复调用为更新节点总结信息
    pub fn upsert_summary(&mut self) {
        let summary = NodeSummary::update(self);
        self.summary = summary;
    }
}

// --------------------- 单元测试 ---------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let node = TreeNode::new("./tests/examples/tree/summary".to_string(), true);
        println!("{}", node);
    }

    #[test]
    /// 测试节点创建
    fn test_new() {
        let node = TreeNode::new("./tests/examples/tree/summary".to_string(), true);
        assert_eq!(node.path, "./tests/examples/tree/summary");
        assert_eq!(node.is_dir, true);
        assert!(node.children.is_some());
        // 检查总结信息
        assert_eq!(node.summary.size, 0);
        assert_eq!(node.summary.count, 0);
    }

    #[test]
    /// 在测试中调用
    fn test_summary_update() {
        let mut node = TreeNode::new("./tests/examples/tree/summary", true);
        node.upsert_summary();
        println!("{}", node);
    }
}
