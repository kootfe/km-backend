use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub password: String,
    pub password2: String,
}

impl RegisterForm {
    pub fn is_passes_same(&self) -> bool { self.password == self.password2 }
}
