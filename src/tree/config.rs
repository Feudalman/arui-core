use derive_builder::Builder;

/// 配置
#[derive(Default, Debug, Builder, PartialEq, Clone)]
#[builder(default, setter(into))]
pub struct ProjectConfig {
    /// 需要包含的路径
    pub include: Vec<String>,

    /// 需要排除的路径
    pub exclude: Vec<String>,
}

impl ProjectConfig {
    /// 以默认值填充创建一个项目配置对象
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加单个 include（接受 &str 或 String）
    pub fn add_include<S: Into<String>>(mut self, include: S) -> Self {
        self.include.push(include.into());
        self
    }

    /// 添加需要被包含的路径
    pub fn add_includes<I, S>(mut self, includes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.include.extend(includes.into_iter().map(Into::into));
        self
    }

    /// 添加单个 exclude（接受 &str 或 String）
    pub fn add_exclude<S: Into<String>>(mut self, exclude: S) -> Self {
        self.exclude.push(exclude.into());
        self
    }

    /// 添加需要被忽略的路径
    pub fn add_excludes<I, S>(mut self, excludes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.exclude.extend(excludes.into_iter().map(Into::into));
        self
    }

    /// 清空需要被包含的路径
    pub fn clear_include(mut self) -> Self {
        self.include.clear();
        self
    }

    /// 清空需要被忽略的路径
    pub fn clear_exclude(mut self) -> Self {
        self.exclude.clear();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 测试初始化，使用默认值创建成功
    fn test_init_config() {
        let config = ProjectConfig::new();
        println!("{:?}", config);
        assert_eq!(config.include.len(), 0);
        assert_eq!(config.exclude.len(), 0);
    }

    #[test]
    // &str 类型初始化
    fn test_width_include() {
        let config = ProjectConfig::new()
            .add_includes(["./src"])
            .add_excludes(["./node_modules", "./dist"]);
        println!("{:?}", config);
        assert_eq!(config.include.len(), 1);
        assert_eq!(config.exclude.len(), 2);
        let config = config.add_include("123").add_exclude("321");
        println!("{:?}", config);
        assert_eq!(config.include.len(), 2);
        assert_eq!(config.exclude.len(), 3);
    }

    #[test]
    // String 类型初始化
    fn test_width_include_string() {
        let config = ProjectConfig::new()
            .add_includes(vec!["./src".to_string()])
            .add_excludes(vec!["./node_modules".to_string(), "./dist".to_string()]);
        println!("{:?}", config);
        assert_eq!(config.include.len(), 1);
        assert_eq!(config.exclude.len(), 2);
        let config = config.add_include("123".to_string()).add_exclude("321".to_string());
        println!("{:?}", config);
        assert_eq!(config.include.len(), 2);
        assert_eq!(config.exclude.len(), 3);
    }

    #[test]
    // 测试清空 include 和 exclude
    fn test_clear() {
        let mut config = ProjectConfig::new()
            .add_excludes(["123", "1231"])
            .add_includes(["1111"]);
        config = config.clear_include().clear_exclude();
        println!("{:?}", config);
        assert_eq!(config.include.len(), 0);
        assert_eq!(config.exclude.len(), 0);
    }
}