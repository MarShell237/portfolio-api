use entities::users;
use fake::{
    Dummy, Fake, Faker,
    faker::{
        internet::fr_fr::{SafeEmail, Username},
        phone_number::fr_fr::PhoneNumber,
    },
};
use portfolio_api::helpers::hash;
use sea_orm::ActiveValue::{NotSet, Set};

#[derive(Dummy)]
pub struct UserFactory {
    #[dummy(faker = "Username()")]
    pub name: String,
    #[dummy(faker = "SafeEmail()")]
    pub email: String,
    #[dummy(faker = "PhoneNumber()")]
    pub phone: String,
}

pub fn create() -> users::ActiveModel {
    let user_factory: UserFactory = Faker.fake();
    users::ActiveModel {
        id: NotSet,
        picture: NotSet,
        name: Set(user_factory.name),
        email: Set(user_factory.email),
        phone: Set(user_factory.phone),
        email_verified_at: NotSet,
        password: Set(hash::make("password".to_string()).unwrap()),
        deleted_at: NotSet,
        created_at: NotSet,
        updated_at: NotSet,
    }
}
