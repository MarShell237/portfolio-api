use crate::{
    dtos::requests::register_request::RegisterRequestValidated, enums::user_role::UserRole,
    errors::AppError, helpers::hash,
};
use entities::users;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait, DatabaseConnection, EntityTrait,
    QueryFilter,
};

pub async fn create_user(
    register_request: RegisterRequestValidated,
    db_pool: &DatabaseConnection,
) -> Result<users::Model, AppError> {
    let role_str: &'static str = UserRole::VISITOR.into();
    let user = users::ActiveModel {
        name: Set(register_request.name),
        email: Set(register_request.email),
        phone: Set(register_request.phone),
        password: Set(hash::make(register_request.password)?),
        role: Set(role_str.to_string()),
        ..Default::default()
    }
    .insert(db_pool)
    .await?;
    Ok(user)
}

pub async fn find_user_by_id(
    user_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<users::Model, AppError> {
    users::Entity::find_by_id(user_id)
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("User not found"))
}

pub async fn has_role<C>(user_id: i64, user_role: UserRole, db_pool: &C) -> Result<bool, AppError>
where
    C: ConnectionTrait,
{
    let role_str: &'static str = user_role.into();
    Ok(users::Entity::find_by_id(user_id)
        .filter(users::COLUMN.role.eq(role_str))
        .one(db_pool)
        .await?
        .is_some())
}

pub async fn get_first_admin(db_pool: &DatabaseConnection) -> Result<users::Model, AppError> {
    let role_str: &'static str = UserRole::ADMIN.into();
    users::Entity::find()
        .filter(users::COLUMN.role.eq(role_str))
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Admin not found"))
}
