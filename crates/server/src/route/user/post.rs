use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use prisma_client_rust as prisma;

use crate::{db, Db};

#[derive(serde::Deserialize)]
pub struct Req {
    name: String,
}

pub async fn post(
    State(db): State<Db>,
    Json(req): Json<Req>,
) -> Result<Json<db::user::Data>, Error> {
    let user = db.user().create(req.name, vec![]).exec().await?;

    Ok(Json::from(user))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to create user")]
    PrismaError(#[from] prisma::QueryError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Error::PrismaError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        status.into_response()
    }
}
