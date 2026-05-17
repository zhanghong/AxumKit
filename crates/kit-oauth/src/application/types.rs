use kit_entity::common::OAuthProvider;
use kit_entity::users::Model as UserModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthStateData {
    pub pkce_verifier: String,
    pub flow: crate::api::dto::request::OAuthAuthorizeFlow,
    pub provider: OAuthProvider,
    pub anonymous_user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingSignupData {
    pub provider: OAuthProvider,
    pub provider_user_id: String,
    pub anonymous_user_id: String,
    pub email: String,
    pub profile_image: Option<String>,
}

#[derive(Debug)]
pub struct OAuthUserResult {
    pub user: UserModel,
    pub is_new_user: bool,
}

#[derive(Debug)]
pub enum SignInResult {
    Success(String),
    PendingSignup { pending_token: String, email: String },
}
