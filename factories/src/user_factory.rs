use entities::users;
use fake::{
    Dummy, Fake, Faker,
    faker::{
        internet::fr_fr::{Password, SafeEmail, Username},
        phone_number::fr_fr::PhoneNumber,
    },
};
use sea_orm::ActiveValue::{NotSet, Set};

#[derive(Dummy)]
pub struct UserFactory {
    #[dummy(faker = "Username()")]
    pub name: String,
    #[dummy(faker = "SafeEmail()")]
    pub email: String,
    #[dummy(faker = "PhoneNumber()")]
    pub phone: String,
    #[dummy(faker = "Password(8..12)")]
    pub password: String,
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
        password: Set(user_factory.password),
        deleted_at: NotSet,
        created_at: NotSet,
        updated_at: NotSet,
    }
}
