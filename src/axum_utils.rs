use axum::response::IntoResponse;
use http::StatusCode;

#[derive(Debug)]
pub struct AxumError(pub anyhow::Error);

pub type AxumResult<T, E=AxumError> = Result<T, E>;

impl<E> From<E> for AxumError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

impl IntoResponse for AxumError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {:?}", self.0),
        )
            .into_response()
    }
}