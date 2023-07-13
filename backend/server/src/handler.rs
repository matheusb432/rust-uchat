use std::path::PathBuf;

use axum::{
    async_trait,
    body::Full,
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use hyper::{header, StatusCode};
use serde::Deserialize;
use uchat_domain::ids::ImageId;
use uuid::Uuid;

use crate::{
    error::{ApiErr, ApiResult},
    extractor::{DbConnection, UserSession},
    AppState,
};

pub mod post;
pub mod user;

const USER_CONTENT_DIR: &str = "usercontent";

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

#[async_trait]
pub trait AuthorizedApiRequest {
    type Response: IntoResponse;
    async fn process_request(
        self,
        conn: DbConnection,
        session: UserSession,
        state: AppState,
    ) -> ApiResult<Self::Response>;
}

pub async fn with_handler<'a, Req>(
    conn: DbConnection,
    session: UserSession,
    State(state): State<AppState>,
    Json(payload): Json<Req>,
) -> ApiResult<Req::Response>
where
    Req: AuthorizedApiRequest + Deserialize<'a>,
{
    payload.process_request(conn, session, state).await
}

pub async fn save_image<T>(id: ImageId, data: T) -> Result<(), ApiErr>
where
    T: AsRef<[u8]>,
{
    // NOTE tokio::fs enables async file operations
    use tokio::fs;

    let mut path = PathBuf::from(USER_CONTENT_DIR);
    fs::create_dir_all(&path).await?;
    path.push(id.to_string());
    fs::write(&path, data).await?;

    Ok(())
}

const PARSE_ERR: &str =
    "Failed to parse image data url, it must be in a 'data:text/plain;base64' format";

fn parse_img_data_url(raw: &str) -> Result<(&str, &str), ApiErr> {
    // NOTE Using a closure makes it easier to refactor this to have more specific error handling if necessary
    let parse = move || {
        let (header, image_data) = raw.split_once(',')?;
        // ? header: data:text/plain;base64
        let mime_type = header.split_once("data:")?.1.split_once(";base64")?.0;

        Some((image_data, mime_type))
    };

    parse().ok_or_else(|| ApiErr::from_msg(PARSE_ERR))
}

pub async fn load_image(
    Path(img_id): Path<Uuid>,
) -> Result<Response<Full<axum::body::Bytes>>, ApiErr> {
    use tokio::fs;

    let mut path = PathBuf::from(USER_CONTENT_DIR);
    path.push(img_id.to_string());

    let raw = fs::read_to_string(path).await?;

    let (image_data, mime_type) = parse_img_data_url(&raw)?;
    {
        use base64::{engine::general_purpose, Engine as _};
        let image_data = general_purpose::STANDARD.decode(image_data).unwrap();

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime_type)
            .body(Full::from(image_data))
            .unwrap())
    }
}
