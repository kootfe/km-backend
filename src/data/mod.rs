pub mod app;
pub use app::*;

use serde::{Serialize};

#[derive(Serialize)]
pub struct KfResponse<T> {
    pub success: bool,
    pub error: Option<String>,
    pub data: Option<T>,
    pub err_code: Option<i32>,
}

impl<T> KfResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            error: None,
            data: Some(data),
            err_code: None
        }
    }
}

impl KfResponse<()> {
    pub fn err(msg: impl Into<String>) -> Self {
        KfResponse {
            success: false,
            error: Some(msg.into()),
            data: None,
            err_code: None
        }
    }

    pub fn err_c(code: i32, msg: impl Into<String>) -> Self {
        KfResponse {
            success: false,
            error: Some(msg.into()),
            data: None,
            err_code: Some(code)
        }
    }

    pub fn suc() -> Self {
        KfResponse {
            success: true,
            error: None,
            data: None,
            err_code: None
        }
    }
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub createdat: chrono::DateTime<chrono::Utc>, 
}
