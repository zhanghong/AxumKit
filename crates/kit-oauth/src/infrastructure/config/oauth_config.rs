use kit_config::ServerConfig;
use kit_entity::common::OAuthProvider;

pub trait OAuthProviderConfig {
    const AUTH_URL: &'static str;
    const TOKEN_URL: &'static str;
    const SCOPES: &'static [&'static str];
    const PROVIDER: OAuthProvider;
    fn credentials() -> (&'static str, &'static str, &'static str);
}

pub struct GoogleProvider;
impl OAuthProviderConfig for GoogleProvider {
    const AUTH_URL: &'static str = "https://accounts.google.com/o/oauth2/v2/auth";
    const TOKEN_URL: &'static str = "https://oauth2.googleapis.com/token";
    const SCOPES: &'static [&'static str] = &["email", "profile"];
    const PROVIDER: OAuthProvider = OAuthProvider::Google;
    fn credentials() -> (&'static str, &'static str, &'static str) {
        let config = ServerConfig::get();
        (&config.google_client_id, &config.google_client_secret, &config.google_redirect_uri)
    }
}

pub struct GithubProvider;
impl OAuthProviderConfig for GithubProvider {
    const AUTH_URL: &'static str = "https://github.com/login/oauth/authorize";
    const TOKEN_URL: &'static str = "https://github.com/login/oauth/access_token";
    const SCOPES: &'static [&'static str] = &["read:user", "user:email"];
    const PROVIDER: OAuthProvider = OAuthProvider::Github;
    fn credentials() -> (&'static str, &'static str, &'static str) {
        let config = ServerConfig::get();
        (&config.github_client_id, &config.github_client_secret, &config.github_redirect_uri)
    }
}
