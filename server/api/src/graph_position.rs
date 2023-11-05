use entity::graph_position::Model;
use salvo::{Router, handler, Depot, Request, prelude::StatusError, writing::Json};
use service::grapg_position::{Mutation, Query};

use crate::AppState;

type Result<T> = std::result::Result<T, StatusError>;

pub fn get_route() -> Router {
  Router::new().path("graph").post(create).get(get_by_agend_id)
    .push(
      Router::new().path("<position_id>")
      .post(update)
    )
}

#[handler]
async fn get_by_agend_id(req: &mut Request, depot: &mut Depot) -> Result<Json<Vec<Model>>> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;

    let agenda_id = req.query::<i32>("agenda_id");
    println!("id: {agenda_id:?}");

    let position = Query::find_by_agenda_id(&state.conn, agenda_id)
    .await
    .expect("Cannot find tasks in page");

    let result = if let Some(position) = position {
        vec![position]
    } else {
        vec![]
    };
  Ok(Json(result))
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

    let result = Mutation::create_graph_position(conn, form)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            StatusError::internal_server_error()
        })?;
    Ok(Json(result.try_into().unwrap()))
}


#[handler]
async fn update(req: &mut Request, depot: &mut Depot) -> Result<Json<Model>> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;
    let conn = &state.conn;
    let id = req.param::<i32>("position_id").unwrap_or_default();
    let form = req
        .parse_json::<Model>()
        .await
        .map_err(|_| StatusError::bad_request())?;

    let position = Mutation::update_graph_position_by_id(conn, id, form)
        .await
        .map_err(|_| StatusError::internal_server_error())?;
    Ok(Json(position.try_into().unwrap()))
}
