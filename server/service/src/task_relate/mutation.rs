use common::time::current_time_stamp;
use ::entity::{task_relate, task_relate::Entity as TaskRelates};
use sea_orm::*;

pub struct TaskRelateMutation;

impl TaskRelateMutation {
    pub async fn create_task_relate(
        db: &DbConn,
        form_data: task_relate::Model,
    ) -> Result<task_relate::ActiveModel, DbErr> {
        let current_time = current_time_stamp();
        task_relate::ActiveModel {
            parent_id: Set(form_data.parent_id),
            child_id: Set(form_data.child_id),
            belong_agenda_id: Set(form_data.belong_agenda_id),
            create_time: Set(current_time),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn delete_task_relate(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let task_relate: task_relate::ActiveModel = TaskRelates::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find task_relate.".to_owned()))
            .map(Into::into)?;

            task_relate.delete(db).await
    }

    pub async fn delete_all_task_relate(db: &DbConn) -> Result<DeleteResult, DbErr> {
        TaskRelates::delete_many().exec(db).await
    }
}
