use entities::comments;
use fake::faker::lorem::fr_fr::Sentences;
use fake::{Dummy, Fake, Faker};
use sea_orm::ActiveValue::{NotSet, Set};

#[derive(Dummy)]
pub struct CommentFactory {
    #[dummy(faker = "Sentences(4..8)")]
    pub content: Vec<String>,
}

pub fn create() -> comments::ActiveModel {
    let comment_factory: CommentFactory = Faker.fake();

    comments::ActiveModel {
        id: NotSet,
        commenter_id: NotSet,
        parent_id: NotSet,
        commentable_type: NotSet,
        commentable_id: NotSet,
        attachment: NotSet,
        content: Set(comment_factory.content.join(" ")),
        created_at: NotSet,
        updated_at: NotSet,
    }
}
