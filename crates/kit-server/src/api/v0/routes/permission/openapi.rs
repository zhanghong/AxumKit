use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::v0::routes::permission::handler::add_permission,
        crate::api::v0::routes::permission::handler::remove_permission,
        crate::api::v0::routes::permission::handler::get_role_permissions,
    ),
    components(
        schemas(
            kit_dto::permission::request::AddPermissionRequest,
            kit_dto::permission::request::RemovePermissionRequest,
            kit_dto::permission::request::GetRolePermissionsRequest,
            kit_dto::permission::response::RolePermissionsResponse,
        )
    ),
    tags(
        (name = "permission", description = "Permission management endpoints"),
    )
)]
pub struct PermissionApi;
