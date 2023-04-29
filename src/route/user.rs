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

#[derive(serde::Deserialize)]
pub struct CreateInput {
    name: String,
    surname: String,
}

#[derive(serde::Deserialize)]
pub struct PatchInput {
    name: Option<String>,
    surname: Option<String>,
}

#[axum::debug_handler]
pub async fn get(State(db): State<Database>) -> Result<Json<Vec<db::user::Data>>> {
    let users = db
        .user()
        .find_many(vec![])
        .exec()
        .await
        .wrap_err("Failed to get users from the database")?;
    Ok(Json::from(users))
}

#[axum::debug_handler]
pub async fn get_by_id(
    Path(id): Path<uuid::Uuid>,
    State(db): State<Database>,
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

#[axum::debug_handler]
pub async fn post(
    State(db): State<Database>,
    Json(input): Json<CreateInput>,
) -> Result<Json<db::user::Data>> {
    let user = db
        .user()
        .create(input.name, input.surname, vec![])
        .exec()
        .await
        .wrap_err("Failed to insert new user into the database")?;
    Ok(Json::from(user))
}

#[axum::debug_handler]
pub async fn put(
    State(db): State<Database>,
    Path(id): Path<uuid::Uuid>,
    Json(input): Json<CreateInput>,
) -> Result<Json<db::user::Data>> {
    let user = db
        .user()
        .update(
            db::user::id::equals(id.to_string()),
            vec![
                db::user::name::set(input.name),
                db::user::surname::set(input.surname),
            ],
        )
        .exec()
        .await
        .wrap_err("Failed to update user in the database")?;

    Ok(Json::from(user))
}

#[axum::debug_handler]
pub async fn patch(
    State(db): State<Database>,
    Path(id): Path<uuid::Uuid>,
    Json(input): Json<PatchInput>,
) -> Result<Json<db::user::Data>> {
    let user = db
        .user()
        .update(
            db::user::id::equals(id.to_string()),
            std::iter::empty()
                .chain(input.name.map(db::user::name::set))
                .chain(input.surname.map(db::user::surname::set))
                .collect(),
        )
        .exec()
        .await
        .wrap_err("Failed to update user in the database")?;

    Ok(Json::from(user))
}
