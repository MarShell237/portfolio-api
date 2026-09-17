use crate::errors::AppError;
use entities::{roles, users_roles};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectionTrait};

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
