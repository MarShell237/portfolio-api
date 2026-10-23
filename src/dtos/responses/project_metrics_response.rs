use serde::Serialize;

#[derive(Serialize)]
pub struct ProjectMetricsResponse {
    pub project_id: i64,
    pub comments_count: u64,
    pub likes_count: u64,
    pub shares_count: u64,
}
