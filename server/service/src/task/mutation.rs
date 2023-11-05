use common::time::current_time_stamp;
use ::entity::{task, task::Entity as Task};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_task(
        db: &DbConn,
        form_data: task::Model,
    ) -> Result<task::ActiveModel, DbErr> {
        let current_time = current_time_stamp();
        task::ActiveModel {
            title: Set(form_data.title.to_owned()),
            content: Set(form_data.content.to_owned()),
            create_time: Set(current_time),
            edit_time: Set(current_time),
            status: Set(form_data.status),
            belong_agenda_id: Set(form_data.belong_agenda_id),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_task_by_id(
        db: &DbConn,
        id: i32,
        form_data: task::Model,
    ) -> Result<task::Model, DbErr> {
        let task: task::ActiveModel = Task::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find task.".to_owned()))
            .map(Into::into)?;

        task::ActiveModel {
            id: task.id,
            title: Set(form_data.title.to_owned()),
            content: Set(form_data.content.to_owned()),
            create_time: task.create_time,
            edit_time: Set(current_time_stamp()),
            deadline: Set(form_data.deadline),
            status: Set(form_data.status),
            belong_agenda_id: Set(form_data.belong_agenda_id),
        }
        .update(db)
        .await
    }

    pub async fn delete_task(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let task: task::ActiveModel = Task::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find task.".to_owned()))
            .map(Into::into)?;

        task.delete(db).await
    }

    pub async fn delete_all_tasks(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Task::delete_many().exec(db).await
    }
}
