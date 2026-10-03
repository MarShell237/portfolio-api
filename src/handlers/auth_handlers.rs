use crate::{
    dtos::responses::user_response::UserResponse, helpers::api_response::ApiResponse,
    repositories::user_repositories,
};
use actix_web::{Responder, web::Data};
use actix_web_validator::Json;
use entities::users;

use crate::{
    dtos::requests::register_request::RegisterRequest, errors::AppError,
    helpers::app_state::AppState,
};

pub async fn register(
    app_state: Data<AppState>,
    Json(user_request): Json<RegisterRequest>,
) -> Result<impl Responder, AppError> {
    let user: users::Model =
        user_repositories::create_user(user_request, &app_state.db_pool).await?;
    let user_response = UserResponse::from(user);
    Ok(ApiResponse::created(
        format!(
            "Félicitations, {} ! Votre compte a été créé avec succès. Un email de confirmation a été envoyé à l’adresse que vous avez fournie. Veuillez ouvrir cet email et cliquer sur le lien “Vérifier mon adresse e-mail” pour activer votre compte. Si vous ne recevez pas l’email dans quelques minutes, vérifiez votre dossier spam ou demandez à renvoyer le lien de vérification.",
            user_response.name
        ),
        Some(user_response),
    ))
}
