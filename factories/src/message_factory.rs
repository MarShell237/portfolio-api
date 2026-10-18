use entities::messages;
use fake::faker::lorem::fr_fr::Words;
use fake::{
    Dummy, Fake, Faker,
    faker::{
        internet::fr_fr::{SafeEmail, Username},
        phone_number::fr_fr::PhoneNumber,
    },
};
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveValue::Set, entity::prelude::DateTime};

#[derive(Dummy)]
pub struct MessageFactory {
    #[dummy(faker = "Username()")]
    pub name: String,
    #[dummy(faker = "SafeEmail()")]
    pub email: String,
    #[dummy(faker = "PhoneNumber()")]
    pub phone: String,
    #[dummy(faker = "Words(20..60)")]
    pub subject: Vec<String>,
    #[dummy(faker = "Words(200..600)")]
    pub message: Vec<String>,
    pub read_at: Option<DateTime>,
}

pub fn create() -> messages::ActiveModel {
    let messagage_factory: MessageFactory = Faker.fake();
    messages::ActiveModel {
        id: NotSet,
        name: Set(messagage_factory.name),
        phone: Set(messagage_factory.phone),
        email: Set(messagage_factory.email),
        subject: Set(messagage_factory.subject.join("")),
        message: Set(messagage_factory.message.join("")),
        read_at: Set(messagage_factory.read_at),
        ..Default::default()
    }
}
