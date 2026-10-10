use actix_web::{
    HttpMessage,
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};
use entities::users;
use sea_orm::EntityTrait;

use crate::{errors::AppError, helpers::app_state::AppState};

pub async fn verified(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, actix_web::Error> {
    let user_id_str = req
        .extensions()
        .get::<String>()
        .cloned()
        .ok_or_else(|| AppError::unauthorized("Authentification requise."))?;

    let user_id = user_id_str
        .parse::<i64>()
        .map_err(|_| AppError::unauthorized("Identifiant utilisateur invalide."))?;

    let app_state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| AppError::internal("Erreur de configuration de l'application."))?;

    let user = users::Entity::find_by_id(user_id)
        .one(&app_state.db_pool)
        .await
        .map_err(|e| AppError::internal(format!("Erreur base de données: {e}")))?
        .ok_or_else(|| AppError::unauthorized("Utilisateur introuvable."))?;

    if user.email_verified_at.is_none() {
        return Err(AppError::forbidden(
            "Veuillez vérifier votre adresse e-mail pour accéder à cette ressource.",
        )
        .into());
    }

    next.call(req).await
}
