use actix_files as fs;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use std::path::Path;

/// Development server for serving static files
pub struct DevServer {
    pub static_dir: String,
    pub port: u16,
    pub host: String,
}

impl DevServer {
    pub fn new(static_dir: String, port: u16, host: String) -> Self {
        Self {
            static_dir,
            port,
            host,
        }
    }

    /// Start the development server
    pub async fn start(&self) -> std::io::Result<()> {
        let static_dir = self.static_dir.clone();
        let bind_address = format!("{}:{}", self.host, self.port);

        println!("🌐 Starting development server...");
        println!("📁 Serving: {}", static_dir);
        println!("🔗 Local: http://{}", bind_address);
        println!("⏹️  Press Ctrl+C to stop");

        // Check if static directory exists
        if !Path::new(&static_dir).exists() {
            eprintln!("❌ Static directory '{}' does not exist!", static_dir);
            eprintln!("💡 Run 'dodge build' first to generate the site.");
            std::process::exit(1);
        }

        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .wrap(middleware::NormalizePath::trim())
                .service(
                    web::scope("")
                        .service(fs::Files::new("/", &static_dir).index_file("index.html"))
                )
                .default_service(web::route().to(not_found))
        })
        .bind(&bind_address)?
        .run()
        .await
    }
}

/// 404 handler
async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound()
        .content_type("text/html")
        .body(include_str!("../templates/404.html")))
}