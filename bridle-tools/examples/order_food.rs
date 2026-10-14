use async_trait::async_trait;
use bridle_core::tool::{Tool, ToolCall, ToolDefinition, ToolError};
use bridle_tools::registry::ToolRegistry;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;

pub struct OrderFoodTool;

#[derive(Debug, Deserialize)]
struct OrderFoodArgs {
    dish_name: String,

    quantity: u32,
}

#[tokio::main]
async fn main() {
    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(OrderFoodTool));

    let definitions = registry.definitions();
    println!("发送给LLM的工具说明书:");
    for d in &definitions {
        println!(
            "{}",
            serde_json::to_string_pretty(d).expect("tool definition is JSON-serializable")
        );
    }

    let call = ToolCall {
        id: "call_001".to_owned(),
        name: "order_food".to_owned(),
        arguments: json!({ "dish_name": "kung_pao_chicken", "quantity": 2 }),
    };

    let result = registry.dispatch(call).await;
    println!("反馈给LLM的结果: {:?}", result);
}

#[async_trait]
impl Tool for OrderFoodTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "order_food".to_string(),
            description: "Places a food order at a restaurant. \
                Use this tool only after the user has clearly stated \
                a dish name and quantity. Do not invent a dish name \
                on your own."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "dish_name": {
                        "type": "string",
                        "description": "Name of the dish, for example kung_pao_chicken"
                    },
                    "quantity": {
                        "type": "integer",
                        "description": "Number of portions, at least 1",
                        "minimum": 1
                    }
                },
                "required": ["dish_name", "quantity"]
            }),
        }
    }

    fn validate(&self, arguments: &Value) -> Result<(), ToolError> {
        let args: OrderFoodArgs = serde_json::from_value(arguments.clone()).map_err(|e| {
            ToolError::InvalidArgumentType {
                field: "arguments".to_owned(),
                expected: e.to_string(),
            }
        })?;

        if args.quantity == 0 {
            return Err(ToolError::InvalidArgumentType {
                field: "quantity".to_owned(),
                expected: "integer greater than 0".to_owned(),
            });
        }

        Ok(())
    }

    async fn execute(&self, arguments: Value) -> Result<String, ToolError> {
        let args: OrderFoodArgs = serde_json::from_value(arguments)
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        Ok(format!(
            "order confirmed: {} portion(s) of {}",
            args.quantity, args.dish_name,
        ))
    }
}
