use anyhow::Result;
use crate::generator::SiteGenerator;
use crate::theme::Theme;
use crate::config::Config;
use crate::server::DevServer;
use super::Commands;

pub struct CommandHandler;

impl CommandHandler {
    pub async fn handle_command(command: Commands) -> Result<()> {
        match command {
            Commands::Build { input, output, clean } => {
                Self::handle_build(input, output, clean).await
            }
            Commands::Serve { dir, port, host, build, input } => {
                Self::handle_serve(dir, port, host, build, input).await
            }
            Commands::Clean { output } => {
                Self::handle_clean(output).await
            }
        }
    }

    async fn handle_build(input: String, output: String, clean: bool) -> Result<()> {
        let config = Config::load().unwrap_or_default();
        let theme = config.theme.parse::<Theme>().unwrap_or(Theme::Hacker);
        let generator = SiteGenerator::new(input, output);
        
        if clean {
            generator.clean()?;
        }
        
        generator.build_with_config(&config.blog_title, &theme)?;
        Ok(())
    }

    async fn handle_serve(dir: String, port: u16, host: String, build: bool, input: String) -> Result<()> {
        if build {
            println!("🔨 Auto-building site before serving...");
            let generator = SiteGenerator::new(input, dir.clone());
            generator.build()?;
            println!();
        }
        
        let server = DevServer::new(dir, port, host);
        server.start().await?;
        Ok(())
    }

    async fn handle_clean(output: String) -> Result<()> {
        let generator = SiteGenerator::new("".to_string(), output);
        generator.clean()?;
        Ok(())
    }
}