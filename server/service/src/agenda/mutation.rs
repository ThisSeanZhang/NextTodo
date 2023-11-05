use common::time::current_time_stamp;
use ::entity::{agenda, agenda::Entity as Agenda};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_agenda(
        db: &DbConn,
        form_data: agenda::Model,
    ) -> Result<agenda::ActiveModel, DbErr> {
        let current_time = current_time_stamp();
        agenda::ActiveModel {
            icon: Set(form_data.icon.to_owned()),
            name: Set(form_data.name.to_owned()),
            description: Set(form_data.description.to_owned()),
            create_time: Set(current_time),
            default_status: Set(form_data.default_status),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_agenda_by_id(
        db: &DbConn,
        id: i32,
        form_data: agenda::Model,
    ) -> Result<agenda::Model, DbErr> {
        let agenda: agenda::ActiveModel = Agenda::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find agenda.".to_owned()))
            .map(Into::into)?;

        agenda::ActiveModel {
            id: agenda.id,
            icon: Set(form_data.icon.to_owned()),
            name: Set(form_data.name.to_owned()),
            description: Set(form_data.description.to_owned()),
            create_time: agenda.create_time,
            default_status: Set(form_data.default_status),
        }
        .update(db)
        .await
    }

    pub async fn delete_agenda(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let agenda: agenda::ActiveModel = Agenda::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find agenda.".to_owned()))
            .map(Into::into)?;

            agenda.delete(db).await
    }

    pub async fn delete_all_agendas(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Agenda::delete_many().exec(db).await
    }
}
