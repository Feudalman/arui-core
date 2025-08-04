//! Summary
//! 负责计算和汇总节点所在的总结信息。
//! 该模块仅维护数据层面的字段和接口，而不负责数据获取等等操作。
//! 对于不同的节点，计算规则如下：
//! - 文件：直接计算
//! - 目录：统计目录下所有文件的累加
use crate::tree::node::TreeNode;
use crate::tree::node::count::get_file_count;
use crate::tree::node::file::get_file_size;
use std::fmt::Display;

#[derive(Debug, Clone)]
/// 节点总结信息
pub struct NodeSummary {
    // 磁盘占用大小
    pub size: u64,
    // 包含文本行数
    pub count: u64,
    // 最后更新时间
    pub updated_at: Option<std::time::SystemTime>,
    // 包含的文件后缀
    pub suffixes: Vec<String>,
}

impl Display for NodeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n  size: {},\n  count: {},\n  updated_at: {:?},\n  suffixes: {:?}",
            self.size, self.count, self.updated_at, self.suffixes
        )
    }
}

impl NodeSummary {
    /// 创建节点
    pub fn new() -> Self {
        NodeSummary {
            size: 0,
            count: 0,
            updated_at: None,
            suffixes: Vec::new(),
        }
    }

    /// 更新节点信息
    /// 任意节点可以为文件，也可以为目录，所以我们需要对两种情况都做判断，并使用不同的分支处理
    ///
    /// ### 文件
    /// 如果节点为文件，直接调用文件的总结函数然后赋值即可
    ///
    /// ### 目录
    /// 如果节点为目录，则需要递归的调用子节点的总结函数，由底向上调用总结信息，直到根节点
    /// 在每次发现节点是目录时，不对其调用总结函数，而是继续递归调用子节点的总结函数
    /// 直到最后子项遍历完毕，目录节点对子节点进行累加得到最终结果。
    pub fn update(node: &mut TreeNode) -> NodeSummary {
        let mut summary = NodeSummary::new();
        summary.updated_at = Some(std::time::SystemTime::now());

        if !node.is_dir {
            summary.size = get_file_size(&node.path).unwrap_or(0);
            summary.count = get_file_count(&node.path).unwrap_or(0);
            return summary;
        }

        if let Some(children) = &mut node.children {
            for child in children {
                // 递归调用子节点
                let child_summary = NodeSummary::update(child);
                // 写入子节点
                child.summary = child_summary.clone();

                // 累加到父节点
                summary.size += child_summary.size;
                summary.count += child_summary.count;
                // 如果需要合并 suffixes，可加上这行：
                // summary.suffixes.extend(child_summary.suffixes.clone());
            }
        }

        summary
    }
}

// --------------------- 单元测试 ---------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::node::TreeNode;

    #[test]
    fn test_display() {
        let ns = NodeSummary::new();
        println!("{}", ns);
    }

    #[test]
    fn test_create_summary() {
        let ns = NodeSummary::new();
        assert_eq!(ns.size, 0);
        assert_eq!(ns.count, 0);
        assert_eq!(ns.updated_at, None);
        assert_eq!(ns.suffixes.len(), 0);
    }

    #[test]
    fn test_summary_with_file() {
        // 创建节点
        let mut node = TreeNode::new("./tests/examples/tree/summary/test.txt", false);
        // 启动节点总结
        node.upsert_summary();
        println!("{}", node);
    }

    #[test]
    fn test_summary_with_dir() {
        // 创建节点
        let mut node = TreeNode::new("./tests/examples/tree/summary", true);
        let sub_node_1 = TreeNode::new("./tests/examples/tree/summary/test.txt", false);
        let sub_node_2 = TreeNode::new("./tests/examples/tree/summary/test.rs", false);
        let sub_node_3 = TreeNode::new("./tests/examples/tree/summary/test.js", false);
        // 模拟子节点
        node.children = Some(vec![sub_node_1, sub_node_2, sub_node_3]);
        // 启动节点总结
        node.upsert_summary();
        // 打印子节点信息
        if let Some(children) = &node.children {
            for child in children {
                println!("{}", child.summary);
            }
        }
        // 打印父节点信息
        println!("{}", node);
    }
}
