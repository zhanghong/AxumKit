use kit_errors::errors::Errors;
use reqwest::Client as HttpClient;
use serde::Deserialize;

const GOOGLE_USER_INFO_URL: &str = "https://www.googleapis.com/oauth2/v2/userinfo";

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub picture: String,
}

pub async fn fetch_google_user_info(http_client: &HttpClient, access_token: &str) -> Result<GoogleUserInfo, Errors> {
    let response = http_client.get(GOOGLE_USER_INFO_URL).bearer_auth(access_token).send().await.map_err(|_| Errors::OauthUserInfoFetchFailed)?;
    if !response.status().is_success() { return Err(Errors::OauthUserInfoFetchFailed); }
    let response_text = response.text().await.map_err(|_| Errors::OauthUserInfoFetchFailed)?;
    let user_info = serde_json::from_str::<GoogleUserInfo>(&response_text).map_err(|_| Errors::OauthUserInfoParseFailed(response_text))?;
    Ok(user_info)
}
