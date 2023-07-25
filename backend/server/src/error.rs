use crate::prelude::*;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use uchat_endpoint::RequestFailed;

pub type ApiResult<T> = std::result::Result<T, ApiErr>;

pub struct ApiErr {
    pub code: Option<StatusCode>,
    pub err: color_eyre::Report,
}

impl ApiErr {
    pub fn new<T: Into<String>>(code: StatusCode, err: T) -> Self {
        let msg: String = err.into();
        Self {
            code: Some(code),
            err: color_eyre::Report::new(RequestFailed { msg }),
        }
    }

    pub fn from_msg<T: Into<String>>(msg: T) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, msg)
    }
}

pub fn err_response<T: Into<String>>(code: StatusCode, msg: T) -> Response {
    (
        code,
        Json(uchat_endpoint::RequestFailed { msg: msg.into() }),
    )
        .into_response()
}

impl IntoResponse for ApiErr {
    fn into_response(self) -> axum::response::Response {
        if let Some(code) = self.code {
            return err_response(code, format!("{}", self.err));
        }
        tracing::error!("{}", self.err);
        err_response(StatusCode::INTERNAL_SERVER_ERROR, "server error")
    }
}

// NOTE Blanket implementation for all types that have a `Into<color_eyre::Report>` impl.
// ? So that anything that can be converted into a `color_eyre::Report` can be converted into an `ApiErr`
impl<E> From<E> for ApiErr
where
    E: Into<color_eyre::Report>,
{
    fn from(err: E) -> Self {
        Self {
            code: None,
            err: err.into(),
        }
    }
}
