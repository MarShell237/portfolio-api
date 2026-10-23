use crate::{
    dtos::{forms::register_form::RegisterForm, requests::login_request::{LoginRequest, LoginRequestValidated}, responses::user_response::UserResponse}, helpers::{api_response::ApiResponse, hash}, repositories::user_repositories,
};
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_web::{HttpMessage, HttpRequest, Responder, web::{Data, ReqData}};
use actix_web_validator::Json;
use entities::users;

use crate::{
    dtos::requests::register_request::RegisterRequest, errors::AppError,
    helpers::app_state::AppState,
};

pub async fn register(
    app_state: Data<AppState>,
    http_request: HttpRequest,
    request: MultipartForm<RegisterForm>,
) -> Result<impl Responder, AppError> {
    let (register_request_validated, photo) = request.into_inner().validated_and_split(&app_state.db_pool).await?; 
    let user: users::Model =
        user_repositories::create_user(register_request_validated, &app_state.db_pool).await?;
    let user_response = UserResponse::from_user(user, &app_state.db_pool).await?;
    Identity::login(
        &http_request.extensions(),
        user_response.id.clone().to_string().into(),
    )
    .map_err(|e| AppError::internal(format!("Échec de création de la session: {e}")))?;
    Ok(ApiResponse::created(
        format!(
            "Félicitations, {} ! Votre compte a été créé avec succès. Un email de confirmation a été envoyé à l’adresse que vous avez fournie. Veuillez ouvrir cet email et cliquer sur le lien “Vérifier mon adresse e-mail” pour activer votre compte. Si vous ne recevez pas l’email dans quelques minutes, vérifiez votre dossier spam ou demandez à renvoyer le lien de vérification.",
            user_response.name
        ),
        Some(user_response),
    ))
}

pub async fn login(
    app_state: Data<AppState>,
    http_request: HttpRequest,
    Json(login_request): Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let login_request_validated = LoginRequestValidated::from(login_request);
    let user = users::Entity::find_by_email(login_request_validated .email)
        .one(&app_state.db_pool)
        .await?
        .ok_or_else(|| AppError::unauthorized(   
            "Identifiants invalides. Veuillez vérifier votre adresse e-mail et votre mot de passe, puis réessayer.",
        ))?;
    hash::check(login_request_validated.password, user.clone().password)?;

    Identity::login(
        &http_request.extensions(),
        user.id.clone().to_string().into(),
    )
    .map_err(|e| AppError::internal(format!("Échec de création de la session: {e}")))?;

    let user_response = UserResponse::from_user(user, &app_state.db_pool).await?;
    Ok(ApiResponse::ok(
        format!(
            "Connexion réussie. Bienvenue, {}, sur votre espace utilisateur.",
            user_response.name
        ),
        Some(user_response),
    ))
}

pub async fn logout(user: Option<Identity>) -> Result<impl Responder, AppError>{
    let session = user.ok_or_else(|| {
        AppError::unauthorized("Authentification requise.")
    })?;
    session.logout();
    Ok(ApiResponse::ok("Vous avez été déconnecté de votre session avec succès.", None::<()>))
}

pub async fn connected(user_id: ReqData<i64>, app_state: Data<AppState>) -> Result<impl Responder, AppError> {
    let user = user_repositories::find_user_by_id(*user_id, &app_state.db_pool).await?;
    let user_response = UserResponse::from_user(user, &app_state.db_pool).await?;
    Ok(ApiResponse::ok("Utilisateur connecté récupéré avec succès.", Some(user_response)))
}
