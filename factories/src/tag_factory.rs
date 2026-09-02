use entities::tags as tag;
use fake::faker::color::raw::*;
use fake::faker::lorem::fr_fr::*;
use fake::locales::FR_FR;
use fake::{Dummy, Fake, Faker};
use sea_orm::ActiveValue::{NotSet, Set};

#[derive(Dummy)]
pub struct TagFactory {
    #[dummy(faker = "Words(3..5)")]
    pub name: Vec<String>,

    #[dummy(faker = "HexColor(FR_FR)")]
    pub color: String,

    #[dummy(faker = "Sentences(4..8)")]
    pub description: Vec<String>,
}

pub fn create() -> tag::ActiveModel {
    let tag_factory: TagFactory = Faker.fake();

    let name = tag_factory.name.join(" ");
    let slug = slug::slugify(&name);
    tag::ActiveModel {
        id: NotSet,
        name: Set(name),
        slug: Set(slug),
        color: Set(tag_factory.color),
        description: Set(tag_factory.description.join(" ")),
        icon: NotSet,
        created_at: NotSet,
        updated_at: NotSet,
    }
}
