use axum::{async_trait, extract::FromRequestParts, Extension, RequestPartsExt};
use hyper::{http::request::Parts, StatusCode};
use uchat_query::OwnedAsyncConnection;

use crate::AppState;

// NOTE An extractor is essentially dependency injection, it allows us to inject the db connection into the endpoints

/// This extractor provides the db connection to the endpoints
pub struct DbConnection(pub OwnedAsyncConnection);

#[async_trait]
impl<S> FromRequestParts<S> for DbConnection
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let state = parts
            .extract::<Extension<AppState>>()
            .await
            .expect("could not extract state, add it as a layer to the router config");

        let connection = state.db_pool.get_owned().await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to connect to database",
            )
        })?;
        Ok(Self(connection))
    }
}
