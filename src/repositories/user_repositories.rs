use crate::{
    dtos::requests::register_request::RegisterRequest, enums::user_role::UserRole,
    errors::AppError, helpers::hash,
};
use entities::{roles, users, users_roles};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectionTrait, DatabaseConnection};

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
    let user_role = users_roles::ActiveModel {
        user_id: Set(user_id),
        role_id: Set(role.id),
    };
    user_role.insert(db_pool).await?;
    Ok(())
}

pub async fn create_user(
    register_request: RegisterRequest,
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
