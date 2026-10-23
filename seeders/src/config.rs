use std::{env, sync::LazyLock};

pub static ADMIN_NAME: LazyLock<String> =
    LazyLock::new(|| env::var("ADMIN_NAME").expect("ADMIN_NAME must be defined in the .env file"));

pub static ADMIN_PHONE: LazyLock<String> = LazyLock::new(|| {
    env::var("ADMIN_PHONE").expect("ADMIN_PHONE must be defined in the .env file")
});

pub static ADMIN_EMAIL: LazyLock<String> = LazyLock::new(|| {
    env::var("ADMIN_EMAIL").expect("ADMIN_EMAIL must be defined in the .env file")
});

pub static ADMIN_PASSWORD: LazyLock<String> = LazyLock::new(|| {
    env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be defined in the .env file")
});
