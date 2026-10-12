use bridle_core::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    println!("项目启动中!");

    let config = AppConfig::from_env()?;

    println!("mode name: {}", config.model_name);
    println!("api base: {}", config.api_base);

    Ok(())
}
