use anyhow::Result;
use std::fs;
use std::path::Path;
use glob::glob;

pub struct ContentProcessor;

impl ContentProcessor {
    /// Collect all markdown files matching the pattern
    pub fn collect_posts(input_dir: &str) -> Result<Vec<String>> {
        let pattern = format!("{}/**/*.md", input_dir);
        let mut posts = Vec::new();

        for entry in glob(&pattern)? {
            match entry {
                Ok(path) => posts.push(path.to_string_lossy().to_string()),
                Err(e) => eprintln!("⚠️  Error reading path: {}", e),
            }
        }

        posts.sort();
        Ok(posts)
    }

    /// Get output path for a markdown file
    pub fn get_output_path(input_path: &str, input_dir: &str, output_dir: &str) -> String {
        input_path
            .replace(input_dir, output_dir)
            .replace(".md", ".html")
    }

    /// Ensure output directory exists for a file
    pub fn ensure_output_dir(output_path: &str) -> Result<()> {
        if let Some(parent) = Path::new(output_path).parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }
}