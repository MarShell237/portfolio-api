use actix_identity::Identity;
use actix_web::{
    HttpMessage,
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};

use crate::helpers::api_response::ApiResponse;

pub async fn auth(
    identity: Option<Identity>,
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, actix_web::Error> {
    let user_id = identity.and_then(|identity| identity.id().ok());

    match user_id {
        Some(user_id) => {
            req.extensions_mut().insert(user_id);
            next.call(req).await
        }
        None => Ok(req.into_response(
            ApiResponse::<()>::unauthorized("Authentification requise").into_http_response(),
        )),
    }
}
