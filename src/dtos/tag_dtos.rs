use entities::tags::Model as Tag;
use sea_orm::entity::prelude::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TagRequest {
    pub icon: Option<String>,
    pub name: String,
    pub slug: String,
    pub color: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct TagResponse {
    pub id: i64,
    pub icon: Option<String>,
    pub name: String,
    pub slug: String,
    pub color: String,
    pub description: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl From<Tag> for TagResponse {
    fn from(tag: Tag) -> Self {
        Self {
            id: tag.id,
            icon: tag.icon,
            name: tag.name,
            slug: tag.slug,
            color: tag.color,
            description: tag.description,
            created_at: tag.created_at,
            updated_at: tag.updated_at,
        }
    }
}
