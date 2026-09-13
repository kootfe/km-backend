use crate::{
    app::KM,
    data::{
        form::{LoginForm, RegisterForm},
        user::{User, check_hash, hash_password},
    },
    db::users::{get_auth_user_by_name, get_user_by_name, insert_user},
    http::{jwt, response::ApiErrorE},
};
use thiserror::Error;
use tracing::{info, warn, instrument};

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("Username already taken")]
    DuplicateUsername,

    #[error("Invalid username or password")]
    WrongLogin,

    #[error("password hashing failed")]
    HashFail,
    #[error("Internal")]
    Internal,

    #[error("Db error: {0}")]
    Db(#[from] anyhow::Error),
}

impl From<UserServiceError> for ApiErrorE {
    fn from(e: UserServiceError) -> Self {
        match e {
            UserServiceError::DuplicateUsername => ApiErrorE::DuplicateUsername,
            UserServiceError::WrongLogin => ApiErrorE::WrongLogin,
            UserServiceError::HashFail | UserServiceError::Db(_) | UserServiceError::Internal => ApiErrorE::Internal,
        }
    }
}

#[instrument(skip(km, form), fields(username = %form.username))]
pub async fn register_user(km: &KM, form: RegisterForm) -> Result<User, UserServiceError> {
    if !form.is_passes_same() {
        warn!("Password mismatch at register.");
        return Err(UserServiceError::WrongLogin);
    }

    if get_user_by_name(km.pool(), &form.username).await.is_ok() {
        warn!("Username already taken.");
        return Err(UserServiceError::DuplicateUsername);
    }
    let hash = hash_password(&form.password).map_err(|_| UserServiceError::HashFail)?;
    let user = insert_user(km.pool(), &form.username, &hash).await?;
    info!(user_id = %user.id, "user registered.");

    Ok(user)
}

#[instrument(skip(km, form), fields(username = %form.username))]
pub async fn login_user(km: &KM, form: LoginForm) -> Result<String, UserServiceError> {
    let user = get_auth_user_by_name(km.pool(), &form.username).await.map_err(|_| UserServiceError::WrongLogin)?;
    if !check_hash(&form.password, &user) {
        return Err(UserServiceError::WrongLogin);
    };
    let token = jwt::JwtToken::new(user.id, km.jwt_secret(), user.jwt_v, 20000).map_err(|_| UserServiceError::Internal)?;
    info!(user_name = %user.username, "User logged in");
    Ok(token)
}
