#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(jsw_secret: String) -> Self {
        Self {
            jwt_secret: jsw_secret
        }
    }
}
