use async_trait::async_trait;
use serde_json::Value;
use std::fmt;

#[derive(Debug)]
pub enum ToolError {
    InvalidArguments(String),

    ExecutionFailed(String),
}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToolError::InvalidArguments(msg) => write!(f, "参数错误: {}", msg),
            ToolError::ExecutionFailed(msg) => write!(f, "执行失败: {}", msg),
        }
    }
}

impl std::error::Error for ToolError {}

#[async_trait]
pub trait BaseTool: Send + Sync {
    fn name(&self) -> &str;

    fn description(&self) -> &str;

    fn parameters_schema(&self) -> Value;

    async fn execute(&self, arguments: Value) -> Result<String, ToolError>;
}
