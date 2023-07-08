use axum::{async_trait, Json};
// TODO refactor, status code to this crate's prelude?
use hyper::StatusCode;
use tracing::info;
use uchat_endpoint::user::endpoint::{CreateUser, CreateUserOk, Login, LoginOk};

use crate::{error::ApiResult, extractor::DbConnection, AppState};

use super::PublicApiRequest;

#[async_trait]
impl PublicApiRequest for CreateUser {
    type Response = (StatusCode, Json<CreateUserOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let password_hash = uchat_crypto::hash_password(&self.password)?;
        let user_id = uchat_query::user::new(&mut conn, password_hash, &self.username)?;

        info!(username = self.username.as_ref(), "new user created");

        Ok((
            StatusCode::CREATED,
            Json(CreateUserOk {
                username: self.username,
                user_id,
            }),
        ))
    }
}

#[async_trait]
impl PublicApiRequest for Login {
    type Response = (StatusCode, Json<LoginOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        todo!()
    }
}
