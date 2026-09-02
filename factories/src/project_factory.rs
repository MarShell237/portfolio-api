use entities::projects as project;
use fake::faker::lorem::fr_fr::*;
use fake::{Dummy, Fake, Faker};
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::entity::prelude::DateTime;

#[derive(Dummy)]
pub struct ProjectFactory {
    #[dummy(faker = "Words(8..12)")]
    pub title: Vec<String>,
    #[dummy(faker = "Words(20..60)")]
    pub excerpt: Vec<String>,
    #[dummy(faker = "Words(2000..6000)")]
    pub content: Vec<String>,
    #[dummy(faker = "15000..1000000")]
    pub estimated_cost: i64,
    #[dummy(faker = "1..10")]
    pub reading_time: i64,
    #[dummy(faker = "10..100")]
    pub views_count: i64,
    pub is_pinned: bool,
    pub published_at: Option<DateTime>,
}

pub fn create() -> project::ActiveModel {
    let project_factory: ProjectFactory = Faker.fake();

    let title = project_factory.title.join(" ");
    let slug = slug::slugify(&title);

    project::ActiveModel {
        id: NotSet,
        cover_image: NotSet,
        title: Set(title),
        slug: Set(slug),
        excerpt: Set(project_factory.excerpt.join(" ")),
        content: Set(project_factory.content.join(" ")),
        estimated_cost: Set(project_factory.estimated_cost),
        reading_time: Set(project_factory.reading_time),
        views_count: Set(project_factory.views_count),
        is_pinned: Set(project_factory.is_pinned),
        published_at: Set(project_factory.published_at),
        created_at: NotSet,
        updated_at: NotSet,
    }
}
