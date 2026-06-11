pub mod app;
pub use app::*;

use serde::{Serialize};

#[derive(Serialize)]
pub struct KfResponse<T> {
    pub success: bool,
    pub error: Option<String>,
    pub data: Option<T>,
}

impl<T> KfResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            error: None,
            data: Some(data),
        }
    }

}

impl KfResponse<()> {
    pub fn err(msg: impl Into<String>) -> Self {
        KfResponse {
            success: false,
            error: Some(msg.into()),
            data: None,
        }
    }

    pub fn suc() -> Self {
        KfResponse {
            success: true,
            error: None,
            data: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
}
