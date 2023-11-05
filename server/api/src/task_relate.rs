use entity::task_relate::Model;
use salvo::{Router, handler, Depot, Request, prelude::StatusError, writing::Json};
use service::task_relate::{TaskRelateQuery, TaskRelateMutation};

use crate::AppState;

type Result<T> = std::result::Result<T, StatusError>;

pub fn get_route() -> Router {
  Router::new().path("relates").get(relates).post(create)
}

#[handler]
async fn create(req: &mut Request, depot: &mut Depot) -> Result<Json<Model>> {
    println!("{:?}", req);
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;
    let conn = &state.conn;

    let form = req
        .parse_json::<Model>()
        .await
        .map_err(|e| {
            println!("{:?}", e);
            StatusError::bad_request()
        })?;

    let result = TaskRelateMutation::create_task_relate(conn, form)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            StatusError::internal_server_error()
        })?;
    Ok(Json(result.try_into().unwrap()))
}

#[handler]
async fn relates(req: &mut Request, depot: &mut Depot) -> Result<Json<Vec<Model>>> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;

    let task_id = req.query::<i32>("task_id");
    if let Some(task_id) = task_id {
        let tasks =  TaskRelateQuery::find_by_task_id(&state.conn, task_id)
        .await
        .expect("Cannot find tasks in page");
        return  Ok(Json(tasks));
    }
    let agenda_id = req.query::<i32>("agenda_id");
    if let Some(agenda_id) = agenda_id {
        let tasks =  TaskRelateQuery::find_by_agenda_id(&state.conn, agenda_id)
        .await
        .expect("Cannot find tasks in page");
        return  Ok(Json(tasks));
    }
    Ok(Json(vec![]))
}
