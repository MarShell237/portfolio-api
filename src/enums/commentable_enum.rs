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
    #[sea_orm(string_value = "comment")]
    Comment,
}
