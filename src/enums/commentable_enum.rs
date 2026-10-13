use crate::{
    errors::AppError,
    repositories::{post_repositories, project_repositories},
};
use sea_orm::DatabaseConnection;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(255))")]
#[serde(rename_all = "lowercase")]
pub enum CommentableEnum {
    #[sea_orm(string_value = "project")]
    Project,
    #[sea_orm(string_value = "post")]
    Post,
    // #[sea_orm(string_value = "comment")]
    // Comment,
}

impl CommentableEnum {
    pub async fn validate_commentable(
        commentable_type: &str,
        commentable_id: i64,
        db_pool: &DatabaseConnection,
    ) -> Result<(), AppError> {
        let commentable_enum = match commentable_type {
            "project" => Self::Project,
            "post" => Self::Post,
            // "comment" => Self::Comment,
            _ => {
                return Err(AppError::bad_request(format!(
                    "Type de ressource invalide [{}]",
                    commentable_type
                )));
            }
        };

        match commentable_enum {
            Self::Project => {
                project_repositories::find_or_fail(commentable_id, db_pool).await?;
            }
            Self::Post => {
                post_repositories::find_or_fail(commentable_id, db_pool).await?;
            } // Self::Comment => {
              //     comment_repositories::find_or_fail(commentable_id, db_pool).await?;
              // }
        }

        Ok(())
    }
}
