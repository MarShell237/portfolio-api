use actix_identity::Identity;
use actix_web::{
    Responder,
    web::{Data, Path, ReqData},
};
use actix_web_validator::Json;
use entities::users;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ModelTrait};

use crate::{
    dtos::{
        requests::update_user_request::UpdateUserRequest, responses::user_response::UserResponse,
    },
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState},
    repositories::user_repositories,
};

pub async fn show(
    app_state: Data<AppState>,
    user_id: Path<i64>,
) -> Result<impl Responder, AppError> {
    let user = user_repositories::find_user_by_id(user_id.into_inner(), &app_state.db_pool).await?;

    if user.deleted_at.is_some() {
        let deleted_user = UserResponse {
            id: -user.id,
            picture: None,
            name: "Utilisateur supprimé".to_string(),
            email: "utilisateur@supprimer.com".to_string(),
            phone: user.phone,
            email_verified_at: None,
            deleted_at: user.deleted_at,
            created_at: user.created_at,
            updated_at: user.updated_at,
            role: "VISITOR".to_string(),
        };
        return Ok(ApiResponse::ok(
            "Cet utilisateur n'existe plus.",
            Some(deleted_user),
        ));
    }

    let user_response = UserResponse::from(user);
    Ok(ApiResponse::ok(
        "Utilisateur récupéré avec succès.",
        Some(user_response),
    ))
}

pub async fn update(
    user_id: ReqData<String>,
    Json(request): Json<UpdateUserRequest>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let user = user_repositories::find_user_by_id(
        user_id.into_inner().parse().unwrap(),
        &app_state.db_pool,
    )
    .await?;
    let mut user_active_model: users::ActiveModel = user.into();
    user_active_model.name = Set(request.name);
    let new_user = user_active_model
        .update(&app_state.db_pool)
        .await
        .map_err(AppError::from)?;
    let user_response = UserResponse::from(new_user);
    Ok(ApiResponse::ok(
        "Compte mis à jour avec succès.",
        Some(user_response),
    ))
}

pub async fn destroy(
    user_id: ReqData<String>,
    user_session: Option<Identity>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let user = user_repositories::find_user_by_id(
        user_id.into_inner().parse().unwrap(),
        &app_state.db_pool,
    )
    .await?;
    let session = user_session.ok_or_else(|| {
        AppError::unauthorized("Authentification requise.")
    })?;
    session.logout();
    let mut user_active_model: users::ActiveModel = user.into();
    user_active_model.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    let _new_user = user_active_model
        .update(&app_state.db_pool)
        .await
        .map_err(AppError::from)?;
    Ok(ApiResponse::ok(
        "Compte supprimé avec succès.",
        None::<()>,
    ))
}
