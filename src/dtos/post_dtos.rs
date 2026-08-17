use entities::posts::Model as Post;
use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct PostResponse {
    pub id: i64,
    pub cover_image: Option<String>,
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub content: String,
    pub reading_time: i64,
    pub views_count: i64,
    pub is_pinned: bool,
    pub published_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Serialize)]
pub struct PostCollection {
    pub id: i64,
    pub cover_image: Option<String>,
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    // pub content: String,
    pub reading_time: i64,
    pub views_count: i64,
    pub is_pinned: bool,
    pub published_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            cover_image: post.cover_image,
            title: post.title,
            slug: post.slug,
            excerpt: post.excerpt,
            content: post.content,
            reading_time: post.reading_time,
            views_count: post.views_count,
            is_pinned: post.is_pinned,
            published_at: post.published_at,
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}

impl From<Post> for PostCollection {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            cover_image: post.cover_image,
            title: post.title,
            slug: post.slug,
            excerpt: post.excerpt,
            // content: post.content,
            reading_time: post.reading_time,
            views_count: post.views_count,
            is_pinned: post.is_pinned,
            published_at: post.published_at,
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}
