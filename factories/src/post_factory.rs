use entities::posts;
use fake::faker::lorem::fr_fr::*;
use fake::{Dummy, Fake, Faker};
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::entity::prelude::DateTime;

#[derive(Dummy)]
pub struct PostFactory {
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

pub fn create() -> posts::ActiveModel {
    let post_factory: PostFactory = Faker.fake();

    let title = post_factory.title.join(" ");
    let slug = slug::slugify(&title);

    posts::ActiveModel {
        id: NotSet,
        cover_image: NotSet,
        title: Set(title),
        slug: Set(slug),
        excerpt: Set(post_factory.excerpt.join(" ")),
        content: Set(post_factory.content.join(" ")),
        reading_time: Set(post_factory.reading_time),
        views_count: Set(post_factory.views_count),
        is_pinned: Set(post_factory.is_pinned),
        published_at: Set(post_factory.published_at),
        created_at: NotSet,
        updated_at: NotSet,
    }
}
