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