use actix_web::{
    Responder,
    web::{Data, Path, ReqData},
};
use actix_web_validator::Json;

use crate::{
    dtos::requests::share_request::ShareRequest,
    enums::shareable_enum::ShareableEnum,
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState},
    repositories::share_repositories,
};

pub async fn store(
    path: Path<(String, i64)>,
    user_id: ReqData<i64>,
    Json(request): Json<ShareRequest>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let (shareable_type, shareable_id) = path.into_inner();
    ShareableEnum::validate_shareable(shareable_type.as_str(), shareable_id, &app_state.db_pool)
        .await?;
    let message = if share_repositories::already_shared(
        shareable_type.clone(),
        shareable_id,
        *user_id,
        &app_state.db_pool,
    )
    .await?
    {
        "la ressource a deja ete partager par cet utilisateur"
    } else {
        share_repositories::store(
            shareable_type,
            shareable_id,
            *user_id,
            request.platform,
            &app_state.db_pool,
        )
        .await?;
        "partage de la ressource par cet utilisateur enregistrer avec succes"
    };
    Ok(ApiResponse::ok(message, None::<()>))
}
