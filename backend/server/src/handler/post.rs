use axum::{async_trait, Json};
use chrono::{Duration, Utc};
// TODO refactor, status code to this crate's prelude?
use hyper::StatusCode;
use tracing::info;
use uchat_domain::ids::UserId;
use uchat_endpoint::post::endpoint::{NewPost, NewPostOk};
use uchat_query::{post::Post, session::Session};

use crate::{
    error::ApiResult,
    extractor::{DbConnection, UserSession},
    AppState,
};

use super::AuthorizedApiRequest;

#[async_trait]
impl AuthorizedApiRequest for NewPost {
    type Response = (StatusCode, Json<NewPostOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let post = Post::new(session.user_id, self.content, self.options)?;

        let post_id = uchat_query::post::new(&mut conn, post)?;

        Ok((StatusCode::OK, Json(NewPostOk { post_id })))
    }
}
