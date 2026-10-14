use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::tools::{base::ToolError, registry::ToolRegistry};

/// 工具执行完成后的统一结果结构
///
/// 这个结构体会被放入对话历史中，作为一条 role 为 tool 的消息内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// 对应 LLM 发起的那一次工具调用的编号
    pub tool_call_id: String,
    /// 被执行的工具名称，便于日志追踪
    pub tool_name: String,
    /// 执行产出的文本内容，无论成功还是失败都会填充这个字段
    pub content: String,
    /// 标记这次执行是否发生了错误
    pub is_error: bool,
}

impl ToolResult {
    pub fn success(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        content: String,
    ) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            content,
            is_error: false,
        }
    }

    pub fn error(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        error: ToolError,
    ) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            content: error.to_string(),
            is_error: true,
        }
    }
}

pub async fn execute_tool_call(
    registry: &ToolRegistry,
    tool_call_id: &str,
    tool_name: &str,
    arguments: Value,
) -> ToolResult {
    match registry.get(tool_name) {
        Some(tool) => match tool.execute(arguments).await {
            Ok(content) => ToolResult::success(tool_call_id, tool_name, content),
            Err(error) => ToolResult::error(tool_call_id, tool_name, error),
        },
        None => ToolResult::error(
            tool_call_id,
            tool_name,
            ToolError::ExecutionFailed(format!("Tool {} not exists", tool_name)),
        ),
    }
}
