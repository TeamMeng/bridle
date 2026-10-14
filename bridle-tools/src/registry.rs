use std::{collections::HashMap, sync::Arc};

use bridle_core::tool::{Tool, ToolCall, ToolDefinition, ToolResult};

/// 工具注册
#[derive(Default)]
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        let name = tool.definition().name.clone();
        self.tools.insert(name, tool);
    }

    pub fn definitions(&self) -> Vec<ToolDefinition> {
        self.tools.values().map(|t| t.definition()).collect()
    }

    pub async fn dispatch(&self, call: ToolCall) -> ToolResult {
        let tool = match self.tools.get(&call.name) {
            Some(t) => t,
            None => {
                return ToolResult {
                    call_id: call.id,
                    content: format!("tool not found: {}", call.name),
                    is_error: true,
                };
            }
        };

        if let Err(e) = tool.validate(&call.arguments) {
            return ToolResult {
                call_id: call.id,
                content: format!("argument validation failed: {}", e),
                is_error: true,
            };
        }

        match tool.execute(call.arguments).await {
            Ok(output) => ToolResult {
                call_id: call.id,
                content: output,
                is_error: false,
            },
            Err(e) => ToolResult {
                call_id: call.id,
                content: format!("execution error: {}", e),
                is_error: true,
            },
        }
    }
}
