use std::collections::HashMap;

use async_trait::async_trait;
use axum::{
    extract::{rejection::FormRejection, FromRequest},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    Form, Json,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
    B: Send + 'static,
{
    type Rejection = ServerError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(validation_errors) => (
                StatusCode::BAD_REQUEST,
                Json(json!(get_validation_field_errors(validation_errors))),
            ),
            ServerError::AxumFormRejection(_) => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "rejected": self.to_string() })),
            ),
        }
        .into_response()
    }
}

fn get_validation_field_errors(validation_errors: ValidationErrors) -> HashMap<String, String> {
    validation_errors
        .field_errors()
        .iter()
        .map(|(&field, &errors)| {
            let first_error = match errors.first() {
                Some(e) if e.message.is_some() => e.message.as_ref().unwrap().as_ref(),
                Some(_) | None => "Something went wrong",
            };
            (field.to_string(), first_error.to_string())
        })
        .collect()
}
