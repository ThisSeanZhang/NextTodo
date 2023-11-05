use ::entity::{agenda, agenda::Entity as Agenda};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_agenda_by_id(db: &DbConn, id: i32) -> Result<Option<agenda::Model>, DbErr> {
      Agenda::find_by_id(id).one(db).await
    }

    /// If ok, returns (task models, num pages).
    pub async fn find_agendas_in_page(
        db: &DbConn,
        page: u64,
        tasks_per_page: u64,
    ) -> Result<(Vec<agenda::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Agenda::find()
            .order_by_asc(agenda::Column::Id)
            .paginate(db, tasks_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated tasks
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
