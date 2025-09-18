use std::fs;
use anyhow::Result;
use super::Theme;

pub struct ThemeAssets;

impl ThemeAssets {
    /// Copy theme assets (CSS, etc.) to the output directory
    pub fn copy_theme_assets(theme: &Theme, output_dir: &str) -> Result<()> {
        let theme_css = Self::get_theme_css(theme);
        let css_path = format!("{}/assets/style.css", output_dir);
        
        // Create assets directory
        fs::create_dir_all(format!("{}/assets", output_dir))?;
        
        // Write CSS file
        fs::write(css_path, theme_css)?;
        
        Ok(())
    }

    /// Get the CSS content for a specific theme
    fn get_theme_css(theme: &Theme) -> String {
        match theme {
            Theme::Vercel => include_str!("../../themes/vercel.css").to_string(),
            Theme::Hacker => include_str!("../../themes/hacker.css").to_string(),
        }
    }
}