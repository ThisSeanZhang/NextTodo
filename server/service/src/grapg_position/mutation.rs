use ::entity::{graph_position, graph_position::Entity as GraphPosition};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_graph_position(
        db: &DbConn,
        form_data: graph_position::Model,
    ) -> Result<graph_position::ActiveModel, DbErr> {
        graph_position::ActiveModel {
            belong_agenda_id: Set(form_data.belong_agenda_id.to_owned()),
            config: Set(form_data.config.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_graph_position_by_id(
        db: &DbConn,
        id: i32,
        form_data: graph_position::Model,
    ) -> Result<graph_position::Model, DbErr> {
        let graph_position: graph_position::ActiveModel = GraphPosition::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find graph_position.".to_owned()))
            .map(Into::into)?;

        graph_position::ActiveModel {
            id: graph_position.id,
            belong_agenda_id: Set(form_data.belong_agenda_id.to_owned()),
            config: Set(form_data.config.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_graph_position(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let graph_position: graph_position::ActiveModel = GraphPosition::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find graph_position.".to_owned()))
            .map(Into::into)?;

            graph_position.delete(db).await
    }

    pub async fn delete_all_graph_position(db: &DbConn) -> Result<DeleteResult, DbErr> {
        GraphPosition::delete_many().exec(db).await
    }
}
