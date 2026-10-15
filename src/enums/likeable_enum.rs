use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    errors::AppError,
    repositories::{comment_repositories, post_repositories, project_repositories},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(255))")]
#[serde(rename_all = "lowercase")]
pub enum LikeableEnum {
    #[sea_orm(string_value = "project")]
    Project,
    #[sea_orm(string_value = "post")]
    Post,
    #[sea_orm(string_value = "comment")]
    Comment,
}

impl LikeableEnum {
    pub async fn validate_likeable(
        likeable_type: &str,
        likeable_id: i64,
        db_pool: &DatabaseConnection,
    ) -> Result<(), AppError> {
        let likeable_enum = match likeable_type {
            "project" => LikeableEnum::Project,
            "post" => LikeableEnum::Post,
            "comment" => LikeableEnum::Comment,
            _ => {
                return Err(AppError::bad_request(format!(
                    "Type de ressource invalide [{}]",
                    likeable_type
                )));
            }
        };

        match likeable_enum {
            Self::Project => {
                project_repositories::find_or_fail(likeable_id, db_pool).await?;
            }
            Self::Post => {
                post_repositories::find_or_fail(likeable_id, db_pool).await?;
            }
            Self::Comment => {
                comment_repositories::find_or_fail(likeable_id, db_pool).await?;
            }
        }

        Ok(())
    }
}
