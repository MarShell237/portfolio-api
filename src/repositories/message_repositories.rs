use entities::messages;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

use crate::{dtos::requests::message_request::MessageRequest, errors::AppError};

pub async fn create(
    message_request: MessageRequest,
    db_pool: &DatabaseConnection,
) -> Result<messages::Model, AppError> {
    Ok(messages::ActiveModel {
        name: Set(message_request.name),
        email: Set(message_request.email),
        phone: Set(message_request.phone),
        subject: Set(message_request.subject),
        message: Set(message_request.message),
        ..Default::default()
    }
    .insert(db_pool)
    .await?)
}
