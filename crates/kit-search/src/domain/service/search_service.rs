use kit_entity::users;
use serde_json::{Value as JsonValue, json};

/// Build a search document JSON from user model
pub fn build_user_search_json(user: &users::Model) -> JsonValue {
    json!({
        "id": user.id.to_string(),
        "handle": user.handle,
        "display_name": user.display_name,
        "bio": user.bio,
        "profile_image": user.profile_image,
    })
}
