use axum::{
    extract::{Path, State},
    Json,
};
use eyre::WrapErr;

use crate::{
    db::user,
    error::{ApiError, ApiResult},
    Db,
};

pub async fn get_many(State(db): State<Db>) -> ApiResult<Json<Vec<user::Data>>> {
    let users = db
        .user()
        .find_many(vec![])
        .exec()
        .await
        .wrap_err("Failed to get users from the database")?;
    Ok(Json::from(users))
}

pub async fn get_by_id(
    State(db): State<Db>,
    Path(id): Path<uuid::Uuid>,
) -> ApiResult<Json<user::Data>> {
    let user = db
        .user()
        .find_unique(user::id::equals(id.to_string()))
        .exec()
        .await
        .wrap_err("Failed to get user from the database")?;
    let user = user.ok_or(ApiError::NotFound)?;
    Ok(Json::from(user))
}

#[derive(serde::Deserialize)]
pub struct PostReq {
    name: String,
}

pub async fn post(State(db): State<Db>, Json(req): Json<PostReq>) -> ApiResult<Json<user::Data>> {
    let user = db
        .user()
        .create(req.name, vec![])
        .exec()
        .await
        .wrap_err("Failed to insert new user into the database")?;
    Ok(Json::from(user))
}
