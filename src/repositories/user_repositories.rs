use crate::{
    dtos::requests::register_request::RegisterRequestValidated, enums::user_role::UserRole,
    errors::AppError, helpers::hash, repositories::user_repositories,
};
use entities::{roles, users, users_roles};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait, DatabaseConnection, EntityTrait,
    ModelTrait, PaginatorTrait, QueryFilter,
};

pub async fn assign_role<C>(
    user_id: i64,
    user_role: impl Into<&str>,
    db_pool: &C,
) -> Result<(), AppError>
where
    C: ConnectionTrait,
{
    let role = roles::Entity::find_by_name(user_role.into())
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Role not found"))?;
    users_roles::ActiveModel {
        user_id: Set(user_id),
        role_id: Set(role.id),
    }
    .insert(db_pool)
    .await
    .map_err(AppError::from)
    .map(|_| ())
}

pub async fn create_user(
    register_request: RegisterRequestValidated,
    db_pool: &DatabaseConnection,
) -> Result<users::Model, AppError> {
    let user = users::ActiveModel {
        name: Set(register_request.name),
        email: Set(register_request.email),
        phone: Set(register_request.phone),
        password: Set(hash::make(register_request.password)?),
        ..Default::default()
    }
    .insert(db_pool)
    .await?;
    assign_role(user.id, UserRole::VISITOR, db_pool).await?;
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
    let count = users::Entity::find_by_id(user_id)
        .find_also_related(roles::Entity)
        .filter(roles::COLUMN.name.eq(role_str))
        .count(db_pool)
        .await?;

    Ok(count > 0)
}

pub async fn get_role(
    user_id: i64,
    db_pool: &DatabaseConnection,
) -> Result<roles::Model, AppError> {
    user_repositories::find_user_by_id(user_id, db_pool)
        .await?
        .find_related(roles::Entity)
        .one(db_pool)
        .await?
        .ok_or_else(|| AppError::not_found("Aucun rôle trouvé pour cet utilisateur."))
}

pub async fn get_first_admin(db_pool: &DatabaseConnection) -> Result<users::Model, AppError> {
    let role_str: &'static str = UserRole::ADMIN.into();
    users::Entity::find()
        .find_also_related(roles::Entity)
        .filter(roles::COLUMN.name.eq(role_str))
        .one(db_pool)
        .await?
        .map(|(user, _role)| user)
        .ok_or_else(|| AppError::not_found("Admin not found"))
}
