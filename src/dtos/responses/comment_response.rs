use entities::comments;
use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct CommentResponse {
    pub id: i64,
    pub commenter_id: i64,
    pub parent_id: Option<i64>,
    pub commentable_type: String,
    pub commentable_id: i64,
    pub attachment: Option<String>,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<comments::Model> for CommentResponse {
    fn from(c: comments::Model) -> Self {
        CommentResponse {
            id: c.id,
            commenter_id: c.commenter_id,
            parent_id: c.parent_id,
            commentable_type: c.commentable_type,
            commentable_id: c.commentable_id,
            attachment: c.attachment,
            content: c.content,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}
