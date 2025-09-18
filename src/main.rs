use anyhow::Result;
use clap::{Parser, Subcommand};

mod server;
mod config;

use dodge::{SiteGenerator, Theme};
use config::Config;
use server::DevServer;

#[derive(Parser)]
#[command(name = "dodge")]
#[command(about = "A minimal RAM stack static site generator")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the static site
    Build {
        /// Input directory containing markdown files
        #[arg(short, long, default_value = "content")]
        input: String,
        
        /// Output directory for generated HTML files
        #[arg(short, long, default_value = "public")]
        output: String,
        
        /// Clean output directory before building
        #[arg(long)]
        clean: bool,
    },
    /// Serve the static site with a development server
    Serve {
        /// Directory to serve static files from
        #[arg(short, long, default_value = "public")]
        dir: String,
        
        /// Port to serve on
        #[arg(short, long, default_value = "3000")]
        port: u16,
        
        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        
        /// Auto-build before serving
        #[arg(long)]
        build: bool,
        
        /// Input directory for auto-build
        #[arg(long, default_value = "content")]
        input: String,
    },
    /// Clean the output directory
    Clean {
        /// Output directory to clean
        #[arg(short, long, default_value = "public")]
        output: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { input, output, clean } => {
            let config = Config::load().unwrap_or_default();
            let theme = config.theme.parse::<Theme>().unwrap_or(Theme::Hacker);
            let generator = SiteGenerator::new(input, output);
            
            if clean {
                generator.clean()?;
            }
            
            generator.build_with_config(&config.blog_title, &theme)?;
        }
        
        Commands::Serve { dir, port, host, build, input } => {
            if build {
                println!("🔨 Auto-building site before serving...");
                let generator = SiteGenerator::new(input, dir.clone());
                generator.build()?;
                println!();
            }
            
            let server = DevServer::new(dir, port, host);
            server.start().await?;
        }
        
        Commands::Clean { output } => {
            let generator = SiteGenerator::new("".to_string(), output);
            generator.clean()?;
        }
    }

    Ok(())
}