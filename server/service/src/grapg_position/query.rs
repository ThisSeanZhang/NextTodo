use ::entity::{graph_position, graph_position::Entity as GraphPosition};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_by_agenda_id(db: &DbConn, id: Option<i32>) -> Result<Option<graph_position::Model>, DbErr> {
      let filter = if let Some(id) = id {
        graph_position::Column::BelongAgendaId.eq(id)
      } else {
        graph_position::Column::BelongAgendaId.is_null()
      };
      GraphPosition::find()
      .filter(filter)
      .one(db)
      .await
        
    }

    /// If ok, returns (task models, num pages).
    pub async fn find_agendas_in_page(
        db: &DbConn,
        page: u64,
        tasks_per_page: u64,
    ) -> Result<(Vec<graph_position::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = GraphPosition::find()
            .order_by_asc(graph_position::Column::Id)
            .paginate(db, tasks_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated tasks
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
