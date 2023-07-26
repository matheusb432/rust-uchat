use crate::{
    error::{ApiErr, ServerErr},
    extractor::UserSession,
    prelude::*,
};
use axum::{async_trait, Json};
use chrono::{Duration, Utc};
use tracing::info;
use uchat_domain::{
    ids::{ImageId, UserId},
    user::DisplayName,
};
use uchat_endpoint::{
    user::{
        endpoint::{
            CreateUser, CreateUserOk, FollowUser, FollowUserOk, GetMyProfile, GetMyProfileOk,
            IsFollowing, IsFollowingOk, Login, LoginOk, UpdateProfile, UpdateProfileOk,
            ViewProfile, ViewProfileOk,
        },
        types::{FollowAction, PublicUserProfile},
    },
    Update,
};
use uchat_query::{
    session::Session,
    user::{UpdateProfileParams, User},
};
use url::Url;

use crate::{error::ApiResult, extractor::DbConnection, AppState};

use super::{save_image, AuthorizedApiRequest, PublicApiRequest};

pub struct SessionSignature(String);

pub fn to_public(user: User) -> ApiResult<PublicUserProfile> {
    Ok(PublicUserProfile {
        id: user.id,
        display_name: user
            .display_name
            .and_then(|name| DisplayName::new(name).ok()),
        handle: user.handle,
        profile_image: user.profile_image.as_ref().map(|id| profile_id_to_url(id)),
        created_at: user.created_at,
        am_following: false,
    })
}

fn profile_id_to_url(id: &str) -> Url {
    use uchat_endpoint::app_url::{self, user_content};
    app_url::domain_and(user_content::ROOT)
        .join(user_content::IMAGES)
        .expect("Failed to build root images url, make sure an API_URL environment variable is set")
        .join(id)
        .unwrap()
}

fn new_session(
    state: &AppState,
    conn: &mut uchat_query::AsyncConnection,
    user_id: UserId,
) -> ApiResult<(Session, SessionSignature, Duration)> {
    let fingerprint = serde_json::json!({});
    let session_duration = Duration::weeks(3);
    let session = uchat_query::session::new(conn, user_id, session_duration, fingerprint.into())?;

    let mut rng = state.rng.clone();
    let signature = state
        .signing_keys
        .sign(&mut rng, session.id.as_uuid().as_bytes());

    let signature = uchat_crypto::encode_base64(signature);

    Ok((session, SessionSignature(signature), session_duration))
}

#[async_trait]
impl PublicApiRequest for CreateUser {
    type Response = (StatusCode, Json<CreateUserOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let password_hash = uchat_crypto::hash_password(&self.password)?;
        let user_id = uchat_query::user::new(&mut conn, password_hash, &self.username)
            .map_err(|_| ServerErr::account_exists())?;

        info!(username = self.username.as_ref(), "new user created");

        let (session, signature, duration) = new_session(&state, &mut conn, user_id)?;

        Ok((
            StatusCode::CREATED,
            Json(CreateUserOk {
                username: self.username,
                user_id,
                session_id: session.id,
                session_expires: Utc::now() + duration,
                session_signature: signature.0,
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
        let _span =
            tracing::span!(tracing::Level::INFO, "logging in", user = %self.username.as_ref())
                .entered();

        let check_password = || -> ApiResult<()> {
            let hash = uchat_query::user::get_password_hash(&mut conn, &self.username)?;
            let hash = uchat_crypto::password::deserialize_hash(&hash)?;

            uchat_crypto::verify_password(self.password, &hash)?;
            Ok(())
        };
        check_password().map_err(|_| ServerErr::wrong_password())?;

        let user = uchat_query::user::find(&mut conn, &self.username)
            .map_err(|_| ServerErr::missing_login())?;

        let (session, signature, duration) = new_session(&state, &mut conn, user.id)?;

        let profile_image_url = user.profile_image.as_ref().map(|id| profile_id_to_url(id));

        Ok((
            StatusCode::OK,
            Json(LoginOk {
                session_id: session.id,
                session_expires: Utc::now() + duration,
                session_signature: signature.0,
                display_name: user.display_name,
                email: user.email,
                profile_image: profile_image_url,
                user_id: user.id,
            }),
        ))
    }
}

#[async_trait]
impl AuthorizedApiRequest for GetMyProfile {
    type Response = (StatusCode, Json<GetMyProfileOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        let user = uchat_query::user::get(&mut conn, session.user_id)?;

        let profile_image_url = user.profile_image.as_ref().map(|id| profile_id_to_url(id));

        Ok((
            StatusCode::OK,
            Json(GetMyProfileOk {
                display_name: user.display_name,
                email: user.email,
                profile_image: profile_image_url,
                user_id: user.id,
            }),
        ))
    }
}

#[async_trait]
impl AuthorizedApiRequest for UpdateProfile {
    type Response = (StatusCode, Json<UpdateProfileOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        let password = {
            if let Update::Change(ref password) = self.password {
                Update::Change(uchat_crypto::hash_password(password)?)
            } else {
                Update::NoChange
            }
        };

        let profile_image = if let Update::Change(ref img) = self.profile_image {
            let id = ImageId::new();
            save_image(id, img).await?;
            Update::Change(id.to_string())
        } else {
            self.profile_image
        };

        let query_params = UpdateProfileParams {
            id: session.user_id,
            display_name: self.display_name,
            email: self.email,
            password_hash: password,
            profile_image,
        };

        uchat_query::user::update_profile(&mut conn, query_params)?;

        let profile_image_url = {
            let user = uchat_query::user::get(&mut conn, session.user_id)?;
            user.profile_image.as_ref().map(|id| profile_id_to_url(id))
        };

        Ok((
            StatusCode::OK,
            Json(UpdateProfileOk {
                profile_image: profile_image_url,
            }),
        ))
    }
}

#[async_trait]
impl AuthorizedApiRequest for IsFollowing {
    type Response = (StatusCode, Json<IsFollowingOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        let is_following =
            uchat_query::user::is_following(&mut conn, session.user_id, self.follows)?;

        Ok((StatusCode::OK, Json(IsFollowingOk { is_following })))
    }
}

#[async_trait]
impl AuthorizedApiRequest for FollowUser {
    type Response = (StatusCode, Json<FollowUserOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        if self.follows == session.user_id {
            return Err(ApiErr::new(StatusCode::BAD_REQUEST, "cannot follow self"));
        }

        match self.action {
            FollowAction::Follow => {
                uchat_query::user::follow(&mut conn, session.user_id, self.follows)?;
            }
            FollowAction::Unfollow => {
                uchat_query::user::unfollow(&mut conn, session.user_id, self.follows)?;
            }
        };

        Ok((
            StatusCode::OK,
            Json(FollowUserOk {
                is_following: self.action == FollowAction::Follow,
            }),
        ))
    }
}

#[async_trait]
impl AuthorizedApiRequest for ViewProfile {
    type Response = (StatusCode, Json<ViewProfileOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        let user_id = self.user_id;
        let profile_user = uchat_query::user::get_profile(&mut conn, user_id)?;
        let profile_posts = {
            let posts = uchat_query::post::get_public_posts(&mut conn, user_id)?;
            super::post::many_to_public(&mut conn, posts, Some(&session))
        };
        let is_following = uchat_query::user::is_following(&mut conn, session.user_id, user_id)?;

        Ok((
            StatusCode::OK,
            Json(ViewProfileOk {
                profile: PublicUserProfile {
                    id: profile_user.id,
                    display_name: profile_user
                        .display_name
                        .and_then(|name| DisplayName::new(name).ok()),
                    handle: profile_user.handle,
                    profile_image: profile_user
                        .profile_image
                        .as_ref()
                        .map(|id| profile_id_to_url(id)),
                    created_at: profile_user.created_at,
                    am_following: is_following,
                },
                posts: profile_posts,
            }),
        ))
    }
}
