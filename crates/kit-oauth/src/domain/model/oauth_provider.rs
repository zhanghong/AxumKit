#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OAuthProviderType {
    Google,
    Github,
}

impl OAuthProviderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OAuthProviderType::Google => "google",
            OAuthProviderType::Github => "github",
        }
    }
}

impl std::fmt::Display for OAuthProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
