use reqwest::Client;
use std::{sync::Arc, time::Duration};
use tokio::{sync::Semaphore, time::sleep};

use crate::llm::{
    ChatCompletion, ChatRequest, LlmError,
    types::{AnthropicRequestBody, AnthropicResponseBody, OpenAiRequestBody, OpenAiResponseBody},
};

/// 支持的服务商种类
/// 后续如果要接入新的服务商，只需要在这里加一个枚举值
/// 并在 send_chat_request 里补充对应的分支
#[derive(Debug, Clone)]
pub enum ProviderKind {
    OpenAiCompatible,
    AnthropicCompatible,
}

/// 单个服务商的连接配置
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub kind: ProviderKind,
    pub base_url: String,
    pub api_key: String,
}

/// 统一的 LLM 客户端
/// 所有 Agent 与大模型对话的唯一入口
pub struct LlmClient {
    http: Client,
    provider: ProviderConfig,
    semaphore: Arc<Semaphore>,
    max_retries: u32,
}

impl LlmClient {
    pub fn new(provider: ProviderConfig, max_concurrent: usize) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to build http client");

        LlmClient {
            http,
            provider,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            max_retries: 3,
        }
    }

    pub async fn send_chat_request(
        &self,
        request: ChatRequest,
    ) -> Result<ChatCompletion, LlmError> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .expect("semaphore has been closed unexpectedly");

        let mut attempt = 0;

        loop {
            let result = self.send_once(&request).await;

            match result {
                Ok(completion) => return Ok(completion),

                Err(LlmError::RateLimited(wait_second)) if attempt < self.max_retries => {
                    attempt += 1;

                    let backoff = wait_second.max(1) * (attempt as u64);
                    sleep(Duration::from_secs(backoff)).await;
                    continue;
                }

                Err(other) => return Err(other),
            }
        }
    }

    async fn send_once(&self, request: &ChatRequest) -> Result<ChatCompletion, LlmError> {
        match self.provider.kind {
            ProviderKind::OpenAiCompatible => self.call_openai(request).await,
            ProviderKind::AnthropicCompatible => self.call_anthropic(request).await,
        }
    }

    async fn call_openai(&self, request: &ChatRequest) -> Result<ChatCompletion, LlmError> {
        let body = OpenAiRequestBody {
            model: request.model.clone(),
            messages: request.messages.clone(),
            temperature: request.temperature,
            max_tokens: request.max_tokens,
        };

        let url = format!("{}/v1/chat/completions", self.provider.base_url);

        let response = self
            .http
            .post(&url)
            .bearer_auth(&self.provider.api_key)
            .json(&body)
            .send()
            .await?;

        let status = response.status();

        if status.as_u16() == 429 {
            return Err(LlmError::RateLimited(5));
        }

        if !status.is_success() {
            let message = response.text().await.unwrap_or_default();
            return Err(LlmError::ApiError {
                status: status.as_u16(),
                message,
            });
        }

        let parsed: OpenAiResponseBody = response.json().await?;

        let content = parsed
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content)
            .unwrap_or_default();

        Ok(ChatCompletion {
            content,
            input_tokens: parsed.usage.prompt_tokens,
            output_tokens: parsed.usage.completion_tokens,
        })
    }

    async fn call_anthropic(&self, request: &ChatRequest) -> Result<ChatCompletion, LlmError> {
        let body = AnthropicRequestBody {
            model: request.model.clone(),
            messages: request.messages.clone(),
            temperature: request.temperature,
            max_tokens: request.max_tokens,
        };

        let url = format!("{}/v1/message", self.provider.base_url);

        let response = self
            .http
            .post(&url)
            .header("x-api-key", &self.provider.base_url)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await?;

        let status = response.status();

        if status.as_u16() == 429 {
            return Err(LlmError::RateLimited(5));
        }

        if !status.is_success() {
            let message = response.text().await.unwrap_or_default();
            return Err(LlmError::ApiError {
                status: status.as_u16(),
                message,
            });
        }

        let parsed: AnthropicResponseBody = response.json().await?;

        let content = parsed
            .content
            .into_iter()
            .next()
            .map(|block| block.text)
            .unwrap_or_default();

        Ok(ChatCompletion {
            content,
            input_tokens: parsed.usage.input_tokens,
            output_tokens: parsed.usage.output_tokens,
        })
    }
}
