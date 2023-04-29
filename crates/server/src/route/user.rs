use axum::{
    extract::{Path, State},
    Json,
};
use eyre::WrapErr;

use crate::{
    db,
    route::{Error, Result},
    Database,
};

pub async fn get(State(db): State<Database>) -> Result<Json<Vec<db::user::Data>>> {
    let users = db
        .user()
        .find_many(vec![])
        .exec()
        .await
        .wrap_err("Failed to get users from the database")?;
    Ok(Json::from(users))
}

pub async fn get_by_id(
    State(db): State<Database>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<db::user::Data>> {
    let user = db
        .user()
        .find_unique(db::user::id::equals(id.to_string()))
        .exec()
        .await
        .wrap_err("Failed to get user from the database")?;
    let user = user.ok_or(Error::NotFound)?;
    Ok(Json::from(user))
}

#[derive(serde::Deserialize)]
pub struct PostReq {
    name: String,
}

pub async fn post(
    State(db): State<Database>,
    Json(req): Json<PostReq>,
) -> Result<Json<db::user::Data>> {
    let user = db
        .user()
        .create(req.name, vec![])
        .exec()
        .await
        .wrap_err("Failed to insert new user into the database")?;
    Ok(Json::from(user))
}
