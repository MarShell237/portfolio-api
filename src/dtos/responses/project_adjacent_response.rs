use entities::projects;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProjectAdjacentResponse {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub reading_time: i64,
    pub excerpt: String,
}

#[derive(Serialize)]
pub struct ProjectsAdjacentResponse {
    pub prev: Option<ProjectAdjacentResponse>,
    pub next: Option<ProjectAdjacentResponse>,
}

impl From<projects::Model> for ProjectAdjacentResponse {
    fn from(p: projects::Model) -> Self {
        Self {
            id: p.id,
            title: p.title,
            slug: p.slug,
            reading_time: p.reading_time,
            excerpt: p.excerpt,
        }
    }
}
