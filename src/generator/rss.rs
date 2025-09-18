use anyhow::Result;
use rss::{ChannelBuilder, ItemBuilder};
use std::fs;
use std::path::Path;
use crate::content::MarkdownProcessor;

pub struct RssGenerator;

impl RssGenerator {
    /// Generate RSS feed from markdown posts
    pub fn generate_rss_feed(input_dir: &str, output_dir: &str, posts: &[String]) -> Result<()> {
        let mut items = Vec::new();

        for post_path in posts {
            // Skip non-post files (like index.md, pages/*)
            if !post_path.contains("/posts/") {
                continue;
            }

            let content = fs::read_to_string(post_path)?;
            let html_content = MarkdownProcessor::to_html(&content);
            
            // Extract title from the first line if it's a header
            let title = if content.starts_with("# ") {
                content.lines().next()
                    .unwrap_or("")
                    .trim_start_matches("# ")
                    .to_string()
            } else {
                MarkdownProcessor::extract_title_from_path(post_path)
            };

            // Extract date from filename (assuming format: YYYY-MM-DD-title.md)
            let filename = Path::new(post_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            
            let date_str = if filename.len() >= 10 && filename.chars().nth(4) == Some('-') {
                &filename[..10] // Extract YYYY-MM-DD
            } else {
                "2025-01-01" // Default date
            };

            // Create RSS item
            let link = format!("/{}", post_path
                .replace(input_dir, "")
                .trim_start_matches('/')
                .replace(".md", ".html"));

            let item = ItemBuilder::default()
                .title(Some(title))
                .link(Some(link))
                .description(Some(html_content))
                .pub_date(Some(format!("{} 00:00:00 +0000", date_str)))
                .build();

            items.push(item);
        }

        // Sort items by date (newest first)
        items.sort_by(|a, b| {
            b.pub_date().unwrap_or("").cmp(a.pub_date().unwrap_or(""))
        });

        // Create RSS channel
        let channel = ChannelBuilder::default()
            .title("My Blog")
            .link("http://localhost:3000")
            .description("A blog powered by Dodge SSG")
            .language(Some("en-us".to_string()))
            .items(items)
            .build();

        // Write RSS feed to file
        let rss_content = channel.to_string();
        let rss_path = format!("{}/rss.xml", output_dir);
        fs::write(rss_path, rss_content)?;

        Ok(())
    }


}