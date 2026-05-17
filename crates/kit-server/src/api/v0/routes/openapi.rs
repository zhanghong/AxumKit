use super::action_logs::openapi::ActionLogsOpenApi;
use super::auth::openapi::AuthApiDoc;
use super::moderation::openapi::ModerationOpenApi;
use super::stream::openapi::StreamOpenApi;
use super::user::openapi::UserApiDoc;
use kit_search::SearchApiDoc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi()]
pub struct V0ApiDoc;

impl V0ApiDoc {
    pub fn merged() -> utoipa::openapi::OpenApi {
        let mut openapi = Self::openapi();
        openapi.merge(AuthApiDoc::openapi());
        openapi.merge(UserApiDoc::openapi());
        openapi.merge(SearchApiDoc::openapi());
        openapi.merge(ActionLogsOpenApi::openapi());
        openapi.merge(ModerationOpenApi::openapi());
        openapi.merge(StreamOpenApi::openapi());
        openapi
    }
}
