use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    errors::AppError,
    repositories::{post_repositories, project_repositories},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(255))")]
#[serde(rename_all = "lowercase")]
pub enum ShareableEnum {
    #[sea_orm(string_value = "project")]
    Project,
    #[sea_orm(string_value = "post")]
    Post,
}

impl ShareableEnum {
    pub async fn validate_shareable(
        shareable_type: &str,
        shareable_id: i64,
        db_pool: &DatabaseConnection,
    ) -> Result<(), AppError> {
        let shareable_enum = match shareable_type {
            "project" => ShareableEnum::Project,
            "post" => ShareableEnum::Post,
            _ => {
                return Err(AppError::bad_request(format!(
                    "Type de ressource imvalide [{}]",
                    shareable_type
                )));
            }
        };

        match shareable_enum {
            ShareableEnum::Project => {
                project_repositories::find_or_fail(shareable_id, db_pool).await?;
            }
            ShareableEnum::Post => {
                post_repositories::find_or_fail(shareable_id, db_pool).await?;
            }
        }
        Ok(())
    }
}
