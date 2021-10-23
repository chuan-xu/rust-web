pub mod user;

use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    http::HeaderValue,
    response::IntoResponse,
};
use hyper::{header, Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;

const SEP_DOT: &str = ".";

pub trait ExtJson {
    fn ext_deserialize(self, v: &str) -> Result<Self, String>
    where
        Self: Sized;

    fn ext_serialize(&self) -> String;
}

impl<T> ExtJson for Vec<T>
where
    T: Serialize + DeserializeOwned,
{
    fn ext_deserialize(mut self, val: &str) -> Result<Self, String> {
        let vec_str: Vec<&str> = val.split(SEP_DOT).collect();
        for i in vec_str.iter() {
            let t: T = serde_json::from_str(i).map_err(|e| e.to_string())?;
            self.push(t);
        }
        Ok(self)
    }

    fn ext_serialize(&self) -> String {
        let mut v: Vec<String> = Vec::new();
        for i in self.iter() {
            v.push(serde_json::to_string(i).unwrap());
        }
        v.join(SEP_DOT)
    }
}

//Err-服务错误返回, Privilege-权限限制, Auth-需要认证
#[derive(Deserialize, Serialize, Debug)]
pub enum RspCode {
    Ok,
    Err,
    Privilege,
    Auth,
}

#[derive(Serialize, Debug)]
pub struct Rsp<T: Serialize> {
    ok: bool,
    msg: String,
    code: RspCode,
    data: T,
}

impl<T: Serialize> Rsp<T> {
    pub fn suc_new(data: T) -> Self {
        Rsp {
            ok: true,
            msg: "".to_string(),
            code: RspCode::Ok,
            data,
        }
    }

    pub fn err_new(msg: String, code: RspCode, data: T) -> Self {
        Rsp {
            ok: false,
            msg,
            code,
            data
        }
    }
}

impl<T: Serialize> IntoResponse for Rsp<T> {
    type Body = Full<Bytes>;

    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let bytes = match serde_json::to_vec(&self) {
            Ok(res) => res,
            Err(err) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(header::CONTENT_TYPE, "text/plain")
                    .body(Full::from(err.to_string()))
                    .unwrap();
            }
        };

        let mut res = Response::new(Full::from(bytes));
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        res
    }
}
