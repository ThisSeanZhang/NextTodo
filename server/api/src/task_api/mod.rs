use entity::task::Model;
use salvo::{Request, Depot, handler, Router, prelude::{StatusError, StatusCode}, writing::{Json, Redirect}, Piece, Response};
use service::task::{Query, Mutation};

use crate::AppState;

type Result<T> = std::result::Result<T, StatusError>;

pub fn get_route() -> Router {
//   Router::with_path("projects/<id>")
//   .hoop(filter)
//   .push(
//     Router::with_path("tasks")
//     .get(list)
//     .post(create)
//     .push(Router::new().path("<id>").get(get_one).put(update).delete(delete))
//   )
  
  Router::with_path("tasks")
    .get(list)
    .post(create)
    // TODO: FIX the route
    .push(Router::new().path("title").get(wildcard))
    .push(
        Router::new().path("<task_id>")
        .get(get_one)
        .post(update)
        .delete(delete)
    )
}

#[handler]
async fn filter(req: &mut Request, _depot: &mut Depot, res: &mut Response) -> Result<()> {
    println!("path: {}", req.uri());
    let id = req.param::<i32>("task_id").unwrap_or_default();
    if id == 0 {
        res.status_code(StatusCode::BAD_REQUEST);
    }
  Ok(())
}


#[handler]
async fn list(req: &mut Request, depot: &mut Depot) -> Result<Json<Vec<Model>>> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;
    let agenda_id = req.param::<i32>("agenda_id").unwrap_or_default();

    let _ids: Vec<i32> = req.query("ids").unwrap_or(vec![]);
    let page = req.query("page").unwrap_or(1);
    let tasks_per_page = req
        .query("posts_per_page")
        .unwrap_or(5);

    let (tasks, _num_pages) = Query::find_tasks_in_page(&state.conn, page, agenda_id, vec![], tasks_per_page)
    .await
    .expect("Cannot find tasks in page");

  Ok(Json(tasks))
}

#[handler]
async fn wildcard(req: &mut Request, depot: &mut Depot) -> Result<Json<Vec<Model>>> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;

    let agenda_id = req.param::<i32>("agenda_id").unwrap_or_default();
    let title = req.query::<String>("title").unwrap_or_default();

    let tasks = Query::wildcard_title(&state.conn, agenda_id, title)
    .await
    .expect("Cannot find tasks in page");

  Ok(Json(tasks))
}


#[handler]
async fn get_one(req: &mut Request, depot: &mut Depot) -> Result<Json<Model>> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;

    let id = req.param::<i32>("task_id").unwrap_or_default();
    // let agenda_id = req.param::<i32>("agenda_id").unwrap_or_default();
    println!("id: {id}");

    let task = Query::find_task_by_id(&state.conn, id)
    .await
    .expect("Cannot find tasks in page");

  Ok(Json(task.unwrap()))
}

#[handler]
async fn create(req: &mut Request, depot: &mut Depot, _res: &mut Response) -> Result<()> {
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

    Mutation::create_task(conn, form)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            StatusError::internal_server_error()
        })?;

    // Redirect::found("/").render(res);
    Ok(())
}

#[handler]
async fn update(req: &mut Request, depot: &mut Depot, _res: &mut Response) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;
    let conn = &state.conn;
    let id = req.param::<i32>("task_id").unwrap_or_default();
    let form = req
        .parse_json::<Model>()
        .await
        .map_err(|_| StatusError::bad_request())?;

    Mutation::update_task_by_id(conn, id, form)
        .await
        .map_err(|_| StatusError::internal_server_error())?;

    // Redirect::found("/").render(res);
    Ok(())
}

#[handler]
async fn delete(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;
    let conn = &state.conn;
    let id = req.param::<i32>("task_id").unwrap_or_default();

    Mutation::delete_task(conn, id)
        .await
        .map_err(|_| StatusError::internal_server_error())?;

    Redirect::found("/").render(res);
    Ok(())
}
