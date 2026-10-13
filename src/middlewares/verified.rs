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
    let user_id = req
        .extensions()
        .get::<i64>()
        .cloned()
        .ok_or_else(|| AppError::unauthorized("Authentification requise."))?;

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
            "Votre adresse e-mail n'a pas encore été vérifiée. Pour des raisons de sécurité et pour accéder à cette fonctionnalité, veuillez consulter votre boîte de réception (et vos spams) pour retrouver l'e-mail de confirmation qui vous a été envoyé lors de votre inscription. Si le lien ou le code est expiré, vous pouvez demander l'envoi d'un nouveau lien depuis votre profil.",
        )
        .into());
    }

    next.call(req).await
}
