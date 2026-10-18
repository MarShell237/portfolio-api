use entities::messages;
use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct MessageResponse {
    pub id: i64,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub subject: String,
    pub message: String,
    pub file_path: Option<String>,
    pub read_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<messages::Model> for MessageResponse {
    fn from(m: messages::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            phone: m.phone,
            email: m.email,
            subject: m.subject,
            message: m.message,
            file_path: m.file_path,
            read_at: m.read_at,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}
