use axum::{
    extract,
    response::{Headers, Html, IntoResponse, Json},
};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::{
    dbs::{dbp, DBS},
    mode::{user, Rsp, RspCode},
};

pub async fn test_handler(dbs: extract::Extension<Arc<DBS>>) -> Json<Value> {
    let res: Result<Vec<user::User>, String> = dbs.cache_query(dbp::SQL_QUERY_USERS, vec![]).await;
    match res {
        Ok(r) => {
            let rsp = Rsp::suc_new(r);
            return Json(json!(rsp));
        }
        Err(e) => {
            let rsp = Rsp::err_new(e, RspCode::Err, ());
            return Json(json!(rsp));
        },
    };
}

pub async fn test_json() -> Json<Value> {
    Json(json!({"data": 42}))
}
