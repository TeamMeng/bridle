use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, Serialize)]
pub struct ToolDefinition {
    pub name: String,

    pub description: String,

    pub parameters: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolCall {
    pub id: String,

    pub name: String,

    pub arguments: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolResult {
    pub call_id: String,

    pub content: String,

    pub is_error: bool,
}

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("missing required argument: {0}")]
    MissingArgument(String),

    #[error("invalid argument type for {field}: expected {expected}")]
    InvalidArgumentType { field: String, expected: String },

    #[error("tool execution failed: {0}")]
    ExecutionFailed(String),
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn definition(&self) -> ToolDefinition;

    fn validate(&self, arguments: &Value) -> Result<(), ToolError>;

    async fn execute(&self, arguments: Value) -> Result<String, ToolError>;
}
