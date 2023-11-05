/// Task Relate
/// 1. Associations cannot be recycled
/// 1. 关联不能循环
/// 2. There is a many-to-many relationship between Tasks
/// 2. Task 之间是多对多的关系
/// 

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "task_relates")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    /// source Task ID
    pub parent_id : i32,
    /// destination Task ID
    pub child_id : i32,
    
    pub belong_agenda_id: i32,
    /// for sorting?
    /// 用于排序？
    pub create_time: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
