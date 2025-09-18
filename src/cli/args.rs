use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dodge")]
#[command(about = "A minimal RAM stack static site generator")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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