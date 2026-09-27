use std::env;
use std::sync::LazyLock;

pub static DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
    env::var("DATABASE_URL").expect("DATABASE_URL must be defined in the .env file")
});

pub static REDIS_URL: LazyLock<String> =
    LazyLock::new(|| env::var("REDIS_URL").expect("REDIS_URL must be defined in the .env file"));

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

pub static COOKIE_NAME: LazyLock<String> = LazyLock::new(|| {
    env::var("COOKIE_NAME").expect("COOKIE_NAME must be defined in the .env file")
});

pub static COOKIE_DOMAIN: LazyLock<Option<String>> = LazyLock::new(|| {
    let domain = env::var("COOKIE_DOMAIN").expect("COOKIE_DOMAIN must be defined in the .env file");

    if domain.trim().is_empty() {
        None
    } else {
        Some(domain)
    }
});

pub static COOKIE_SECURE: LazyLock<bool> = LazyLock::new(|| {
    env::var("COOKIE_SECURE")
        .expect("COOKIE_SECURE must be defined in the .env file")
        .parse::<bool>()
        .expect("COOKIE_SECURE must be a boolean ('true' or 'false')")
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
