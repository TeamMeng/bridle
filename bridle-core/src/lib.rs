use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub api_key: String,
    pub api_base: String,
    pub model_name: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let api_key =
            env::var("LLM_API_KEY").map_err(|_| anyhow::anyhow!("LLM_API_KEY not found"))?;
        let api_base =
            env::var("LLM_API_BASE").map_err(|_| anyhow::anyhow!("LLM_API_BASE not found"))?;
        let model_name =
            env::var("LLM_MODEL_NAME").map_err(|_| anyhow::anyhow!("LLM_MODEL_NAME not found"))?;

        Ok(AppConfig {
            api_key,
            api_base,
            model_name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_from_env_fail() {
        temp_env::with_vars(
            [
                ("LLM_API_KEY", None),
                ("LLM_API_BASE", Some("http://localhost:11434/v1")),
                ("LLM_MODEL_NAME", Some("qwen2.5:0.5b")),
            ],
            || {
                let result = AppConfig::from_env();
                assert!(result.is_err());
                assert!(
                    result
                        .unwrap_err()
                        .to_string()
                        .contains("LLM_API_KEY not found")
                );
            },
        )
    }

    #[test]
    fn test_app_config_from_env_success() {
        temp_env::with_vars(
            [
                ("LLM_API_KEY", Some("sk-...")),
                ("LLM_API_BASE", Some("http://localhost:11434/v1")),
                ("LLM_MODEL_NAME", Some("qwen2.5:0.5b")),
            ],
            || {
                let result = AppConfig::from_env();
                assert!(result.is_ok());
            },
        )
    }
}
