use anyhow::Result;
use comrak::{markdown_to_html, ComrakOptions};
use glob::glob;
use std::fs;
use std::path::Path;
use rss::{ChannelBuilder, ItemBuilder};

#[derive(Debug, Clone)]
pub enum Theme {
    Vercel,
    Hacker,
}

impl std::str::FromStr for Theme {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vercel" => Ok(Theme::Vercel),
            "hacker" => Ok(Theme::Hacker),
            _ => Err(format!("Unknown theme: {}. Available themes: vercel, hacker", s)),
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Vercel => write!(f, "vercel"),
            Theme::Hacker => write!(f, "hacker"),
        }
    }
}

/// Static Site Generator functionality
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
        self.copy_theme_assets(&theme)?;

        // Collect and process all markdown files
        let pattern = format!("{}/**/*.md", self.input_dir);
        let posts = self.collect_posts(&pattern)?;

        let mut generated_count = 0;
        for post in posts {
            self.generate_page_with_config(&post, &theme, blog_title)?;
            generated_count += 1;
        }

        // Generate RSS feed
        self.generate_rss_feed()?;

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
        self.copy_theme_assets(theme)?;

        // Collect and process all markdown files
        let pattern = format!("{}/**/*.md", self.input_dir);
        let posts = self.collect_posts(&pattern)?;

        let mut generated_count = 0;
        for post in posts {
            self.generate_page_with_theme(&post, theme)?;
            generated_count += 1;
        }

        // Generate RSS feed
        self.generate_rss_feed()?;

        println!("✅ Generated {} pages successfully!", generated_count);
        println!("📡 Generated RSS feed: /rss.xml");
        Ok(())
    }

    /// Collect all markdown files matching the pattern
    fn collect_posts(&self, pattern: &str) -> Result<Vec<String>> {
        let mut posts = Vec::new();

        for entry in glob(pattern)? {
            match entry {
                Ok(path) => posts.push(path.to_string_lossy().to_string()),
                Err(e) => eprintln!("⚠️  Error reading path: {}", e),
            }
        }

        posts.sort();
        Ok(posts)
    }



    /// Clean the output directory
    pub fn clean(&self) -> Result<()> {
        if Path::new(&self.output_dir).exists() {
            fs::remove_dir_all(&self.output_dir)?;
            println!("🧹 Cleaned output directory: {}", self.output_dir);
        }
        Ok(())
    }

    /// Copy theme assets to output directory
    fn copy_theme_assets(&self, theme: &Theme) -> Result<()> {
        let _theme_dir = format!("themes/{}", theme);
        let css_content = self.get_theme_css(theme);
        
        // Create assets directory
        let assets_dir = format!("{}/assets", self.output_dir);
        fs::create_dir_all(&assets_dir)?;
        
        // Write CSS file
        let css_path = format!("{}/style.css", assets_dir);
        fs::write(css_path, css_content)?;
        
        Ok(())
    }

    /// Get CSS content for a specific theme
    fn get_theme_css(&self, theme: &Theme) -> String {
        match theme {
            Theme::Vercel => include_str!("../themes/vercel.css").to_string(),
            Theme::Hacker => include_str!("../themes/hacker.css").to_string(),
        }
    }

    /// Generate a single page from markdown with theme support
    fn generate_page_with_theme(&self, input_path: &str, theme: &Theme) -> Result<()> {
        let content = fs::read_to_string(input_path)?;
        let html_content = markdown_to_html(&content, &ComrakOptions::default());

        // Wrap content in HTML template
        let full_html = self.wrap_with_template(&html_content, theme, input_path, &content, "My Blog");

        // Create output path
        let output_path = input_path
            .replace(&self.input_dir, &self.output_dir)
            .replace(".md", ".html");

        // Ensure directory exists
        if let Some(parent) = Path::new(&output_path).parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&output_path, full_html)?;
        println!("📄 Generated: {} -> {}", input_path, output_path);
        Ok(())
    }

    /// Generate a single page from markdown with config support
    fn generate_page_with_config(&self, input_path: &str, theme: &Theme, blog_title: &str) -> Result<()> {
        let content = fs::read_to_string(input_path)?;
        let html_content = markdown_to_html(&content, &ComrakOptions::default());

        // Wrap content in HTML template
        let full_html = self.wrap_with_template(&html_content, theme, input_path, &content, blog_title);

        // Create output path
        let output_path = input_path
            .replace(&self.input_dir, &self.output_dir)
            .replace(".md", ".html");

        // Ensure directory exists
        if let Some(parent) = Path::new(&output_path).parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&output_path, full_html)?;
        println!("📄 Generated: {} -> {}", input_path, output_path);
        Ok(())
    }

    /// Wrap content in HTML template
    fn wrap_with_template(&self, content: &str, theme: &Theme, input_path: &str, markdown_content: &str, blog_title: &str) -> String {
        let page_title = self.extract_title_from_markdown(markdown_content, input_path);
        
        let theme_class = match theme {
            Theme::Vercel => "theme-vercel",
            Theme::Hacker => "theme-hacker",
        };

        // Theme-specific header and content structure
        let (header_html, content_html) = match theme {
            Theme::Hacker => {
                let ascii_art = self.generate_ascii_art(blog_title);
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

    /// Extract title from markdown content (H1 header) or fallback to file path
    fn extract_title_from_markdown(&self, markdown_content: &str, path: &str) -> String {
        // Try to extract title from the first H1 header
        for line in markdown_content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                return trimmed.trim_start_matches("# ").trim().to_string();
            }
        }
        
        // Fallback to extracting from file path
        self.extract_title_from_path(path)
    }

    /// Extract title from file path
    fn extract_title_from_path(&self, path: &str) -> String {
        Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .replace("-", " ")
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

    /// Generate RSS feed for blog posts
    fn generate_rss_feed(&self) -> Result<()> {
        let posts = self.collect_posts(&format!("{}/**/*.md", self.input_dir))?;
        let mut items = Vec::new();

        for post_path in posts {
            // Skip non-post files (like index.md, pages/*)
            if !post_path.contains("/posts/") {
                continue;
            }

            let content = fs::read_to_string(&post_path)?;
            let html_content = markdown_to_html(&content, &ComrakOptions::default());
            
            // Extract title from the first line if it's a header
            let title = if content.starts_with("# ") {
                content.lines().next()
                    .unwrap_or("")
                    .trim_start_matches("# ")
                    .to_string()
            } else {
                self.extract_title_from_path(&post_path)
            };

            // Extract date from filename (assuming format: YYYY-MM-DD-title.md)
            let filename = Path::new(&post_path)
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
                .replace(&self.input_dir, "")
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
        let rss_path = format!("{}/rss.xml", self.output_dir);
        fs::write(rss_path, rss_content)?;

        Ok(())
    }

    /// Generate ASCII art for a given title
    fn generate_ascii_art(&self, title: &str) -> String {
        // Generate fancy block letter ASCII art for any title
        self.generate_block_letters(title)
    }

    /// Generate block letter ASCII art for any text
    fn generate_block_letters(&self, text: &str) -> String {
        let text = text.to_uppercase();
        let mut lines = vec![String::new(); 6]; // 6 lines for block letters
        
        for ch in text.chars() {
            let letter_lines = self.get_block_letter(ch);
            for (i, line) in letter_lines.iter().enumerate() {
                if i < lines.len() {
                    lines[i].push_str(line);
                    lines[i].push(' '); // Space between letters
                }
            }
        }
        
        // Remove trailing spaces and join lines
        let result: Vec<String> = lines.iter()
            .map(|line| line.trim_end().to_string())
            .collect();
        
        format!("\n{}\n", result.join("\n"))
    }

    /// Get block letter representation for a single character
    fn get_block_letter(&self, ch: char) -> Vec<&'static str> {
        match ch {
            'A' => vec!["██████╗ ", "██╔══██╗", "███████║", "██╔══██║", "██║  ██║", "╚═╝  ╚═╝"],
            'B' => vec!["██████╗ ", "██╔══██╗", "██████╔╝", "██╔══██╗", "██████╔╝", "╚═════╝ "],
            'C' => vec!["██████╗ ", "██╔════╝", "██║     ", "██║     ", "╚██████╗", " ╚═════╝"],
            'D' => vec!["██████╗ ", "██╔══██╗", "██║  ██║", "██║  ██║", "██████╔╝", "╚═════╝ "],
            'E' => vec!["███████╗", "██╔════╝", "█████╗  ", "██╔══╝  ", "███████╗", "╚══════╝"],
            'F' => vec!["███████╗", "██╔════╝", "█████╗  ", "██╔══╝  ", "██║     ", "╚═╝     "],
            'G' => vec!["██████╗ ", "██╔════╝", "██║  ███╗", "██║   ██║", "╚██████╔╝", " ╚═════╝ "],
            'H' => vec!["██╗  ██╗", "██║  ██║", "███████║", "██╔══██║", "██║  ██║", "╚═╝  ╚═╝"],
            'I' => vec!["██╗", "██║", "██║", "██║", "██║", "╚═╝"],
            'J' => vec!["     ██╗", "     ██║", "     ██║", "██   ██║", "╚█████╔╝", " ╚════╝ "],
            'K' => vec!["██╗  ██╗", "██║ ██╔╝", "█████╔╝ ", "██╔═██╗ ", "██║  ██╗", "╚═╝  ╚═╝"],
            'L' => vec!["██╗     ", "██║     ", "██║     ", "██║     ", "███████╗", "╚══════╝"],
            'M' => vec!["███╗   ███╗", "████╗ ████║", "██╔████╔██║", "██║╚██╔╝██║", "██║ ╚═╝ ██║", "╚═╝     ╚═╝"],
            'N' => vec!["███╗   ██╗", "████╗  ██║", "██╔██╗ ██║", "██║╚██╗██║", "██║ ╚████║", "╚═╝  ╚═══╝"],
            'O' => vec!["██████╗ ", "██╔══██╗", "██║  ██║", "██║  ██║", "╚█████╔╝", " ╚════╝ "],
            'P' => vec!["██████╗ ", "██╔══██╗", "██████╔╝", "██╔═══╝ ", "██║     ", "╚═╝     "],
            'Q' => vec!["██████╗ ", "██╔══██╗", "██║  ██║", "██║ ▄██║", "╚██████╔╝", " ╚═════╝ "],
            'R' => vec!["██████╗ ", "██╔══██╗", "██████╔╝", "██╔══██╗", "██║  ██║", "╚═╝  ╚═╝"],
            'S' => vec!["███████╗", "██╔════╝", "███████╗", "╚════██║", "███████║", "╚══════╝"],
            'T' => vec!["████████╗", "╚══██╔══╝", "   ██║   ", "   ██║   ", "   ██║   ", "   ╚═╝   "],
            'U' => vec!["██╗   ██╗", "██║   ██║", "██║   ██║", "██║   ██║", "╚██████╔╝", " ╚═════╝ "],
            'V' => vec!["██╗   ██╗", "██║   ██║", "██║   ██║", "╚██╗ ██╔╝", " ╚████╔╝ ", "  ╚═══╝  "],
            'W' => vec!["██╗    ██╗", "██║    ██║", "██║ █╗ ██║", "██║███╗██║", "╚███╔███╔╝", " ╚══╝╚══╝ "],
            'X' => vec!["██╗  ██╗", "╚██╗██╔╝", " ╚███╔╝ ", " ██╔██╗ ", "██╔╝ ██╗", "╚═╝  ╚═╝"],
            'Y' => vec!["██╗   ██╗", "╚██╗ ██╔╝", " ╚████╔╝ ", "  ╚██╔╝  ", "   ██║   ", "   ╚═╝   "],
            'Z' => vec!["███████╗", "╚══███╔╝", "  ███╔╝ ", " ███╔╝  ", "███████╗", "╚══════╝"],
            '.' => vec!["   ", "   ", "   ", "   ", "██╗", "╚═╝"],
            ' ' => vec!["    ", "    ", "    ", "    ", "    ", "    "],
            _ => vec!["███╗", "██╔╝", "██║ ", "██║ ", "███╗", "╚══╝"], // Default for unknown chars
        }
    }
}