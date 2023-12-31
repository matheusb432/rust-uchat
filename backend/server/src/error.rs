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

#[derive(Debug, thiserror::Error)]
pub enum ServerErr {
    #[error("Login failed")]
    Login((StatusCode, String)),
    #[error("Registration failed")]
    Registration((StatusCode, String)),
}

impl ServerErr {
    pub fn missing_login() -> Self {
        Self::Login((StatusCode::NOT_FOUND, "Missing login".into()))
    }

    pub fn wrong_password() -> Self {
        Self::Login((StatusCode::BAD_REQUEST, "Invalid password".into()))
    }

    pub fn account_exists() -> Self {
        Self::Login((StatusCode::CONFLICT, "Account already exists".to_string()))
    }
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

        // NOTE downcast_ref tries to convert the error into a reference to the specified type
        if let Some(server_err) = self.err.downcast_ref::<ServerErr>() {
            return match server_err {
                ServerErr::Login((code, msg)) => err_response(*code, msg),
                ServerErr::Registration((code, msg)) => err_response(*code, msg),
            };
        };

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
