use entities::projects::Model as Project;
use sea_orm::entity::prelude::DateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProjectResponse {
    pub id: i64,
    pub cover_image: Option<String>,
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub content: String,
    pub estimated_cost: i64,
    pub reading_time: i64,
    pub views_count: i64,
    pub is_pinned: bool,
    pub published_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Serialize)]
pub struct ProjectCollection {
    pub id: i64,
    pub cover_image: Option<String>,
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    // pub content: String,
    pub estimated_cost: i64,
    pub reading_time: i64,
    pub views_count: i64,
    pub is_pinned: bool,
    pub published_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Serialize)]
pub struct ProjectMetrics {
    pub project_id: i64,
    pub comments_count: u64,
    pub likes_count: u64,
    pub shares_count: u64,
}

#[derive(Serialize)]
pub struct ProjectAdjacent {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub reading_time: i64,
    pub excerpt: String,
}

#[derive(Serialize)]
pub struct ProjectsAdjacentResponse {
    pub prev: Option<ProjectAdjacent>,
    pub next: Option<ProjectAdjacent>,
}

impl From<Project> for ProjectResponse {
    fn from(project: Project) -> Self {
        Self {
            id: project.id,
            cover_image: project.cover_image,
            title: project.title,
            slug: project.slug,
            excerpt: project.excerpt,
            content: project.content,
            estimated_cost: project.estimated_cost,
            reading_time: project.reading_time,
            views_count: project.views_count,
            is_pinned: project.is_pinned,
            published_at: project.published_at,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}

impl From<Project> for ProjectCollection {
    fn from(project: Project) -> Self {
        Self {
            id: project.id,
            cover_image: project.cover_image,
            title: project.title,
            slug: project.slug,
            excerpt: project.excerpt,
            // content: project.content,
            estimated_cost: project.estimated_cost,
            reading_time: project.reading_time,
            views_count: project.views_count,
            is_pinned: project.is_pinned,
            published_at: project.published_at,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}

impl From<Project> for ProjectAdjacent {
    fn from(p: Project) -> Self {
        Self {
            id: p.id,
            title: p.title,
            slug: p.slug,
            reading_time: p.reading_time,
            excerpt: p.excerpt,
        }
    }
}
