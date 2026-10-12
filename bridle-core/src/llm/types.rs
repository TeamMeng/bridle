use serde::{Deserialize, Serialize};

/// 对话中每一条消息的角色
/// system：设定 Agent 的身份和行为准则
/// user：用户发出的输入
/// assistant：模型给出的回答
/// tool：工具执行后返回的结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

/// 统一的消息结构，不管调用哪个服务商，
/// 上层代码始终只构造这一种消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

/// 统一的请求参数
/// 客户端调用方只需要填这一个结构体
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Clone)]
pub struct ChatCompletion {
    pub content: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
}

// ---------- 以下是服务商 A（OpenAI 兼容格式）专用的原始数据结构 ----------
// 这些结构体只在 client.rs 内部使用，不对外暴露
// 它们的字段名和形状严格按照服务商 A 的接口文档来写

#[derive(Debug, Serialize)]
pub(crate) struct OpenAiRequestBody {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OpenAiResponseBody {
    pub choices: Vec<OpenAiChoice>,
    pub usage: OpenAiUsage,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OpenAiChoice {
    pub message: OpenAiMessage,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OpenAiMessage {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OpenAiUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
}

// ---------- 以下是服务商 B（Anthropic 兼容格式）专用的原始数据结构 ----------

#[derive(Debug, Serialize)]
pub(crate) struct AnthropicRequestBody {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: u32,
    pub temperature: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AnthropicResponseBody {
    pub content: Vec<AnthropicContentBlock>,
    pub usage: AnthropicUsage,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AnthropicContentBlock {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AnthropicUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

impl ChatMessage {
    pub fn system(text: impl Into<String>) -> Self {
        ChatMessage {
            role: Role::System,
            content: text.into(),
        }
    }

    pub fn user(text: impl Into<String>) -> Self {
        ChatMessage {
            role: Role::User,
            content: text.into(),
        }
    }

    pub fn assistant(text: impl Into<String>) -> Self {
        ChatMessage {
            role: Role::Assistant,
            content: text.into(),
        }
    }
}
