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
