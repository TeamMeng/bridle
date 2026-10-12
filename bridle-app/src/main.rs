use bridle_core::{
    config::AppConfig,
    llm::{ChatMessage, ChatRequest, LlmClient, ProviderConfig, ProviderKind},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    println!("项目启动中!");

    let app = AppConfig::from_env()?;

    let config = ProviderConfig {
        kind: ProviderKind::OpenAiCompatible,
        base_url: app.api_base,
        api_key: app.api_key,
    };

    let client = LlmClient::new(config, 2);

    let request = ChatRequest {
        model: app.model_name,
        messages: vec![
            ChatMessage::system("你是一个简洁的助手"),
            ChatMessage::user("用一句话介绍并发控制的作用"),
        ],
        temperature: 0.3,
        max_tokens: 200,
    };

    match client.send_chat_request(request).await {
        Ok(completion) => {
            println!("回答内容: {}", completion.content);
            println!(
                "消耗 token 数：输入 {}，输出 {}",
                completion.input_tokens, completion.output_tokens
            );
        }
        Err(e) => {
            eprintln!("调用失败: {}", e);
        }
    }

    Ok(())
}
