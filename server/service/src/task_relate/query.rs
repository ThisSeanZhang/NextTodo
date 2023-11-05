use ::entity::{task_relate, task_relate::Entity as TaskRelates};
use sea_orm::*;

pub struct TaskRelateQuery;

impl TaskRelateQuery {
    pub async fn find_agenda_by_id(db: &DbConn, id: i32) -> Result<Option<task_relate::Model>, DbErr> {
      TaskRelates::find_by_id(id).one(db).await
    }

    pub async fn find_by_task_id(db: &DbConn, id: i32) -> Result<Vec<task_relate::Model>, DbErr> {
        TaskRelates::find()
        .filter(task_relate::Column::ChildId.eq(id).or(task_relate::Column::ParentId.eq(id)))
        .all(db).await
    }

    pub async fn find_by_agenda_id(db: &DbConn, id: i32) -> Result<Vec<task_relate::Model>, DbErr> {
        TaskRelates::find()
        .filter(task_relate::Column::BelongAgendaId.eq(id))
        .all(db).await
    }

    /// If ok, returns (task models, num pages).
    pub async fn find_agendas_in_page(
        db: &DbConn,
        page: u64,
        tasks_per_page: u64,
    ) -> Result<(Vec<task_relate::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = TaskRelates::find()
            .order_by_asc(task_relate::Column::Id)
            .paginate(db, tasks_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated tasks
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
