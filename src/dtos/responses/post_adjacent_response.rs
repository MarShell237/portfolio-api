use entities::posts;
use serde::Serialize;

#[derive(Serialize)]
pub struct PostAdjacentResponse {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub reading_time: i64,
    pub excerpt: String,
}

#[derive(Serialize)]
pub struct PostsAdjacentResponse {
    pub prev: Option<PostAdjacentResponse>,
    pub next: Option<PostAdjacentResponse>,
}

impl From<posts::Model> for PostAdjacentResponse {
    fn from(p: posts::Model) -> Self {
        Self {
            id: p.id,
            title: p.title,
            slug: p.slug,
            reading_time: p.reading_time,
            excerpt: p.excerpt,
        }
    }
}
