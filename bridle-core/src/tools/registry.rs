use serde_json::{Value, json};
use std::{collections::HashMap, sync::Arc};

use crate::tools::base::BaseTool;

#[derive(Default)]
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn BaseTool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, tool: Arc<dyn BaseTool>) {
        let name = tool.name().to_owned();
        self.tools.insert(name, tool);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn BaseTool>> {
        self.tools.get(name).cloned()
    }

    pub fn list_name(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    pub fn to_schema_list(&self) -> Vec<Value> {
        self.tools
            .values()
            .map(|tool| {
                json!({
                    "type": "function",
                    "function": {
                        "name": tool.name(),
                        "description": tool.description(),
                        "parameters": tool.parameters_schema()
                    }
                })
            })
            .collect()
    }
}
