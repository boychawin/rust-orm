// use crate::{messages::FetchUser, AppState, DbActor};
// use actix::Addr;
use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
async fn fetch_users() -> impl Responder {
    const MESSAGE: &str = "0.1.0";

    HttpResponse::Ok().json(json!({ "version": MESSAGE,"description": "test api"}))
}
