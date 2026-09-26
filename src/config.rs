use std::env;
use std::sync::LazyLock;

pub static DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
    env::var("DATABASE_URL").expect("DATABASE_URL must be defined in the .env file")
});

pub static APP_URL: LazyLock<String> =
    LazyLock::new(|| env::var("APP_URL").expect("APP_URL must be defined in the .env file"));

pub static APP_PORT: LazyLock<u16> = LazyLock::new(|| {
    env::var("APP_PORT")
        .expect("APP_PORT must be defined in the .env file")
        .parse()
        .expect("APP_PORT must be an integer")
});

pub static ALLOWED_ORIGIN: LazyLock<String> = LazyLock::new(|| {
    env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN must be defined in the .env file")
});

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
