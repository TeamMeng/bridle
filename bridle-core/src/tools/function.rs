use async_trait::async_trait;
use serde_json::Value;
use std::{pin::Pin, sync::Arc};

use crate::tools::base::{BaseTool, ToolError};

type BoxedFuture = Pin<Box<dyn Future<Output = Result<String, ToolError>> + Send>>;
type ToolFn = Arc<dyn Fn(Value) -> BoxedFuture + Send + Sync>;

pub struct FunctionTool {
    name: String,
    description: String,
    parameters_schema: Value,
    func: ToolFn,
}

impl FunctionTool {
    /// 构造一个新的 FunctionTool
    ///
    /// name: 工具名称
    /// description: 工具描述
    /// parameters_schema: 参数的 JSON Schema
    /// func: 实际执行逻辑的闭包，签名必须是 Fn(Value) -> Future<Output = Result<String, ToolError>>
    pub fn new<F, Fut>(
        name: impl Into<String>,
        description: impl Into<String>,
        parameters_schema: Value,
        func: F,
    ) -> Self
    where
        F: Fn(Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<String, ToolError>> + Send + 'static,
    {
        let wrapped: ToolFn = Arc::new(move |args: Value| -> BoxedFuture { Box::pin(func(args)) });

        Self {
            name: name.into(),
            description: description.into(),
            parameters_schema,
            func: wrapped,
        }
    }
}

#[async_trait]
impl BaseTool for FunctionTool {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn parameters_schema(&self) -> Value {
        self.parameters_schema.clone()
    }

    async fn execute(&self, arguments: Value) -> Result<String, ToolError> {
        // 直接调用持有的闭包，把 execute 的调用转发给真正的业务逻辑
        (self.func)(arguments).await
    }
}
