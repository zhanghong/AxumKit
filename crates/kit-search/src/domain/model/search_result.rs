use serde::Deserialize;

/// Indexed user document stored in MeiliSearch
#[derive(Debug, Deserialize)]
pub struct IndexedUser {
    pub id: String,
    pub handle: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub profile_image: Option<String>,
}
