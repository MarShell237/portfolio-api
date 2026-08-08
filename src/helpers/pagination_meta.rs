use sea_orm::{ConnectionTrait, DbErr, Paginator, SelectorTrait};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PaginationMeta {
    pub current_page: u64,
    pub last_page: u64,
    pub per_page: u64,
    pub total: u64,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
}

impl PaginationMeta {
    pub async fn paginate<'a, C, S>(
        paginator: &'a Paginator<'a, C, S>,
        page: u64,
    ) -> Result<(Vec<S::Item>, Self), DbErr>
    where
        C: ConnectionTrait,
        S: SelectorTrait,
        S::Item: Send + Sync,
    {
        let total = paginator.num_items().await?;
        let last_page = paginator.num_pages().await?;

        let page_idx = if page > 0 { page - 1 } else { 0 };
        let items = paginator.fetch_page(page_idx).await?;

        let meta = Self {
            current_page: page,
            last_page,
            per_page: paginator.cur_page(),
            total,
        };

        Ok((items, meta))
    }
}
