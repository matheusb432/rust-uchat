use axum::{async_trait, extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{error::ApiResult, extractor::DbConnection, AppState};

pub mod user;

#[async_trait]
pub trait PublicApiRequest {
    type Response: IntoResponse;
    async fn process_request(
        self,
        conn: DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response>;
}

/// This handler is used for public endpoints
///
/// It will extract the request payload, the db connection and the app state
///
/// NOTE The `Json` extractor will deserialize the request body into the specified type
///
/// NOTE The `State` extractor will extract the app state from the request
pub async fn with_public_handler<'a, Req>(
    conn: DbConnection,
    State(state): State<AppState>,
    Json(payload): Json<Req>,
) -> ApiResult<Req::Response>
where
    Req: PublicApiRequest + Deserialize<'a>,
{
    payload.process_request(conn, state).await
}