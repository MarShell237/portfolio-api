use entities::messages;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

use crate::{dtos::requests::message_request::MessageRequestValidated, errors::AppError};

pub async fn create(
    message_request_validated: MessageRequestValidated,
    db_pool: &DatabaseConnection,
) -> Result<messages::Model, AppError> {
    Ok(messages::ActiveModel {
        name: Set(message_request_validated.name),
        email: Set(message_request_validated.email),
        phone: Set(message_request_validated.phone),
        subject: Set(message_request_validated.subject),
        message: Set(message_request_validated.message),
        ..Default::default()
    }
    .insert(db_pool)
    .await?)
}
