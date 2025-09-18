use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::theme::{Theme, ThemeAssets};
use crate::utils::AsciiArtGenerator;
use crate::content::{ContentProcessor, MarkdownProcessor};
use super::rss::RssGenerator;

pub struct SiteGenerator {
    pub input_dir: String,
    pub output_dir: String,
}

impl SiteGenerator {
    pub fn new(input_dir: String, output_dir: String) -> Self {
        Self {
            input_dir,
            output_dir,
        }
    }

    /// Generate the entire site
    pub fn build(&self) -> Result<()> {
        self.build_with_theme(&Theme::Vercel)
    }

    pub fn build_with_config(&self, blog_title: &str, theme: &Theme) -> Result<()> {
        println!("🚀 Building site...");
        println!("📁 Input: {}", self.input_dir);
        println!("📁 Output: {}", self.output_dir);
        println!("🎨 Theme: {}", theme);
        println!("📝 Blog Title: {}", blog_title);

        // Create output directory
        fs::create_dir_all(&self.output_dir)?;

        // Copy theme assets (CSS, etc.)
        ThemeAssets::copy_theme_assets(theme, &self.output_dir)?;

        // Collect and process all markdown files
        let posts = ContentProcessor::collect_posts(&self.input_dir)?;

        let mut generated_count = 0;
        for post in &posts {
            self.generate_page_with_config(post, theme, blog_title)?;
            generated_count += 1;
        }

        // Generate RSS feed
        RssGenerator::generate_rss_feed(&self.input_dir, &self.output_dir, &posts)?;

        println!("✅ Generated {} pages successfully!", generated_count);
        println!("📡 Generated RSS feed: /rss.xml");
        Ok(())
    }

    /// Generate the entire site with a specific theme (deprecated - use build() instead)
    pub fn build_with_theme(&self, theme: &Theme) -> Result<()> {
        println!("🚀 Building site...");
        println!("📁 Input: {}", self.input_dir);
        println!("📁 Output: {}", self.output_dir);
        println!("🎨 Theme: {}", theme);

        // Create output directory
        fs::create_dir_all(&self.output_dir)?;

        // Copy theme assets (CSS, etc.)
        ThemeAssets::copy_theme_assets(theme, &self.output_dir)?;

        // Collect and process all markdown files
        let posts = ContentProcessor::collect_posts(&self.input_dir)?;

        let mut generated_count = 0;
        for post in &posts {
            self.generate_page_with_theme(post, theme)?;
            generated_count += 1;
        }

        // Generate RSS feed
        RssGenerator::generate_rss_feed(&self.input_dir, &self.output_dir, &posts)?;

        println!("✅ Generated {} pages successfully!", generated_count);
        println!("📡 Generated RSS feed: /rss.xml");
        Ok(())
    }



    /// Clean the output directory
    pub fn clean(&self) -> Result<()> {
        if Path::new(&self.output_dir).exists() {
            fs::remove_dir_all(&self.output_dir)?;
            println!("🧹 Cleaned output directory: {}", self.output_dir);
        }
        Ok(())
    }

    /// Generate a single page from markdown with theme support
    fn generate_page_with_theme(&self, input_path: &str, theme: &Theme) -> Result<()> {
        let content = fs::read_to_string(input_path)?;
        let html_content = MarkdownProcessor::to_html(&content);

        // Wrap content in HTML template
        let full_html = self.wrap_with_template(&html_content, theme, input_path, &content, "My Blog");

        // Create output path
        let output_path = ContentProcessor::get_output_path(input_path, &self.input_dir, &self.output_dir);

        // Ensure directory exists
        ContentProcessor::ensure_output_dir(&output_path)?;

        fs::write(&output_path, full_html)?;
        println!("📄 Generated: {} -> {}", input_path, output_path);
        Ok(())
    }

    /// Generate a single page from markdown with config support
    fn generate_page_with_config(&self, input_path: &str, theme: &Theme, blog_title: &str) -> Result<()> {
        let content = fs::read_to_string(input_path)?;
        let html_content = MarkdownProcessor::to_html(&content);

        // Wrap content in HTML template
        let full_html = self.wrap_with_template(&html_content, theme, input_path, &content, blog_title);

        // Create output path
        let output_path = ContentProcessor::get_output_path(input_path, &self.input_dir, &self.output_dir);

        // Ensure directory exists
        ContentProcessor::ensure_output_dir(&output_path)?;

        fs::write(&output_path, full_html)?;
        println!("📄 Generated: {} -> {}", input_path, output_path);
        Ok(())
    }

    /// Wrap content in HTML template
    fn wrap_with_template(&self, content: &str, theme: &Theme, input_path: &str, markdown_content: &str, blog_title: &str) -> String {
        let page_title = MarkdownProcessor::extract_title(markdown_content, input_path);
        
        let theme_class = match theme {
            Theme::Vercel => "theme-vercel",
            Theme::Hacker => "theme-hacker",
        };

        // Theme-specific header and content structure
        let (header_html, content_html) = match theme {
            Theme::Hacker => {
                let ascii_art = AsciiArtGenerator::generate_ascii_art(blog_title);
                let header = format!(
                    r#"<div class="ascii-header-container"><div class="ascii-header">{}<br><br>>>> {} <<<"#, 
                    ascii_art, page_title
                );
                let header_complete = format!("{}</div></div>", header);
                (header_complete, content.to_string())
            },
            Theme::Vercel => {
                let header = format!(
                    r#"<header class="site-header">
                        <h1 class="site-title">{}</h1>
                    </header>"#,
                    blog_title
                );
                (header, content.to_string())
            }
        };

        format!(
            r#"<!DOCTYPE html>
<html lang="en" class="{}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <link rel="stylesheet" href="/assets/style.css">
</head>
<body>
    {}
    <main class="container">
        {}
    </main>
</body>
</html>"#,
            theme_class, page_title, header_html, content_html
        )
    }


}