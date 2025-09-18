use comrak::{markdown_to_html, ComrakOptions};
use std::path::Path;

pub struct MarkdownProcessor;

impl MarkdownProcessor {
    /// Convert markdown content to HTML
    pub fn to_html(markdown: &str) -> String {
        markdown_to_html(markdown, &ComrakOptions::default())
    }

    /// Extract title from markdown content (first H1 header)
    pub fn extract_title(markdown_content: &str, fallback_path: &str) -> String {
        // Try to extract title from the first H1 header
        for line in markdown_content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                return trimmed.trim_start_matches("# ").trim().to_string();
            }
        }
        
        // Fallback to extracting from file path
        Self::extract_title_from_path(fallback_path)
    }

    /// Extract title from file path
    pub fn extract_title_from_path(path: &str) -> String {
        Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| {
                // Remove date prefix if present (YYYY-MM-DD-)
                if s.len() > 11 && s.chars().nth(4) == Some('-') && s.chars().nth(7) == Some('-') {
                    &s[11..]
                } else {
                    s
                }
            })
            .unwrap_or("Untitled")
            .replace('-', " ")
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}