use serde::Serialize;

#[derive(Serialize)]
pub struct CommentMeta {
    pub likes_count: u64,
    pub comments_count: u64,
    pub is_liked: bool,
    pub is_liked_by_admin: bool,
    pub has_admin_reply: bool,
}
