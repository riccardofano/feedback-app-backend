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
use validator::{Validate, ValidationError, ValidationErrors};

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
            if let Some(first_error) = errors.first() {
                if let Some(message) = create_error_message(first_error) {
                    return (field.to_string(), message);
                }
            }
            (field.to_string(), "Something went wrong".into())
        })
        .collect()
}

fn create_error_message(error: &ValidationError) -> Option<String> {
    if error.message.is_some() {
        return Some(error.message.as_ref().unwrap().to_string());
    }

    let res = match error.code.as_ref() {
        "email" => "Must be a valid email address".into(),
        "url" => "Must be a valid URL".into(),
        "length" => match_range(&error)? + " characters long",
        "range" => match_range(&error)?,
        "credit_card" => "Must be a valid credit card number".into(),
        "phone" => "Must be a valid phone number".into(),
        "required" => "This field is required".into(),
        _ => return None,
    };

    Some(res)
}

fn match_range(error: &ValidationError) -> Option<String> {
    let mut params: Vec<&str> = error
        .params
        .keys()
        .map(|k| k.as_ref())
        .filter(|k| k != &"value")
        .collect();
    params.sort();

    let res = match params[..] {
        ["max", "min"] => format!(
            "Must be between {} and {}",
            error.params.get("min").unwrap(),
            error.params.get("max").unwrap()
        ),
        ["max"] => format!("Must be at most {}", error.params.get("max").unwrap()),
        ["min"] => format!("Must be at least {}", error.params.get("min").unwrap()),
        ["equal"] => format!("Must be {}", error.params.get("equal").unwrap()),
        _ => return None,
    };

    Some(res)
}
