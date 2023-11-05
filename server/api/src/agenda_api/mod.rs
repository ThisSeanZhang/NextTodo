use entity::agenda::Model;
use salvo::{Request, Depot, handler, Router, prelude::StatusError, writing::Json};
use service::agenda::{Query, Mutation};

use crate::AppState;

type Result<T> = std::result::Result<T, StatusError>;

pub fn get_route() -> Router {
    
    Router::with_path("agendas")
      .get(list)
      .post(create)
      // .push(Router::new().path("<id>").get(get_one).post(update).delete(delete))
      .push(
        Router::new().path("<agenda_id>")
        .push(crate::task_api::get_route())
        .push(crate::task_relate::get_route())
      )
}

#[handler]
async fn list(req: &mut Request, depot: &mut Depot) -> Result<Json<Vec<Model>>> {
  let state = depot
    .obtain::<AppState>()
    .ok_or_else(StatusError::internal_server_error)?;

  let page = req.query("page").unwrap_or(1);
  let agendas_per_page = req
      .query("per_page")
      .unwrap_or(10);

  let (agendas, _num_pages) = Query::find_agendas_in_page(&state.conn, page, agendas_per_page)
  .await
  .expect("Cannot find agendas in page");

  Ok(Json(agendas))
}

#[handler]
async fn create(req: &mut Request, depot: &mut Depot) -> Result<()> {
  
  let state = depot
    .obtain::<AppState>()
    .ok_or_else(StatusError::internal_server_error)?;
  let conn = &state.conn;
  // let txn = conn.begin().await?;
  let form = req
  .parse_json::<Model>()
  .await
  .map_err(|e| {
      println!("{:?}", e);
      StatusError::bad_request()
  })?;

  Mutation::create_agenda(conn, form)
  .await
  .map_err(|e| {
      println!("{:?}", e);
      StatusError::internal_server_error()
  })?;

  Ok(())
}

