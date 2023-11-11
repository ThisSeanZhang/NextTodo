use ::entity::{task, task::Entity as Task};
use sea_orm::*;

pub struct Query;

impl Query {

    pub async fn find_task_by_id(db: &DbConn, id: i32) -> Result<Option<task::Model>, DbErr> {
        Task::find_by_id(id).one(db).await
    }

    
    pub async fn wildcard_title(db: &DbConn,
        belong_agenda_id: i32,
        title: String) -> Result<Vec<task::Model>, DbErr> {
        let query = Task::find()
        .filter(task::Column::BelongAgendaId.eq(belong_agenda_id));
        let query = if title.len() != 0 {
            query.filter(task::Column::Title.contains(&title))
        } else {
            query
        };
        query.all(db).await
    }

    /// If ok, returns (task models, num pages).
    pub async fn find_tasks_in_page(
        db: &DbConn,
        page: u64,
        belong_agenda_id: i32,
        ids: Vec<i32>,
        tasks_per_page: u64,
    ) -> Result<(Vec<task::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Task::find()
            .filter(task::Column::BelongAgendaId.eq(belong_agenda_id));
        let paginator = if ids.len() > 0 {
            paginator.filter(task::Column::Id.is_in(ids))
        } else {
            paginator
        };
        let paginator = paginator.order_by_asc(task::Column::Id)
            .paginate(db, tasks_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated tasks
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn find_all_tasks(
        db: &DbConn,
        belong_agenda_id: i32,
        ids: Vec<i32>,
    ) -> Result<Vec<task::Model>, DbErr> {
        // Setup paginator
        let paginator = Task::find()
            .filter(task::Column::BelongAgendaId.eq(belong_agenda_id));
        let paginator = if ids.len() > 0 {
            paginator.filter(task::Column::Id.is_in(ids))
        } else {
            paginator
        };
       paginator
        .order_by_asc(task::Column::Id)
        .all(db).await
    }
}
