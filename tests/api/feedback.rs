use crate::{
    helpers::{
        create_app, get_request, is_application_json, parse_response_body, patch_request,
        post_request,
    },
    schema::{Feedback, FeedbackForm},
};
use axum::{body::Body, http::StatusCode};
use tower::{Service, ServiceExt};

#[tokio::test]
async fn health_check() {
    let app = create_app().await;

    let request = get_request("/", Body::empty()).await;
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn get_single_feedback() {
    let app = create_app().await;

    let request = get_request("/feedback/1", Body::empty()).await;
    let response = app.oneshot(request).await.unwrap();

    assert!(response.status().is_success());
    assert!(is_application_json(response.headers()));

    let json: Feedback = parse_response_body(response).await;

    assert_eq!(json.id, 1);
}

#[tokio::test]
async fn get_non_existant_feedback() {
    let app = create_app().await;

    let request = get_request("/feedback/999999", Body::empty()).await;
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn post_new_feedback() {
    let app = create_app().await;

    let form = FeedbackForm {
        title: "New feedback".into(),
        category: "bug".into(),
        status: "planned".into(),
        description: "Some description".into(),
    };
    let encoded_form = serde_urlencoded::to_string(&form).unwrap();

    let request = post_request("/feedback/new", encoded_form.into()).await;
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(is_application_json(response.headers()));

    let json: Feedback = parse_response_body(response).await;

    assert_eq!(json.title, form.title);
    assert_eq!(json.category, form.category);
    assert_eq!(json.status, form.status);
    assert_eq!(json.description, form.description);
}

#[tokio::test]
async fn post_new_feedback_with_missing_data() {
    let mut app = create_app().await;

    let encoded_form = "status=planned&category=bug&description=Some%20description";
    let request = post_request("/feedback/new", encoded_form.into()).await;
    let response = app.call(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "expected to be missing title"
    );

    let encoded_form = "title=New%20feedback&category=bug&description=Some%20description";
    let request = post_request("/feedback/new", encoded_form.into()).await;
    let response = app.call(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "expected to be missing status"
    );

    let encoded_form = "title=New%20feedback&status=planned&description=Some%20description";
    let request = post_request("/feedback/new", encoded_form.into()).await;
    let response = app.call(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "expected to be missing category"
    );

    let encoded_form = "title=New%20feedback&status=planned&category=bug";
    let request = post_request("/feedback/new", encoded_form.into()).await;
    let response = app.call(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "expected to be missing description"
    );
}

#[tokio::test]
async fn edit_feedback() {
    let mut app = create_app().await;

    let form = FeedbackForm {
        title: "New feedback".into(),
        category: "bug".into(),
        status: "planned".into(),
        description: "Some description".into(),
    };
    let encoded_form = serde_urlencoded::to_string(&form).unwrap();

    let request = post_request("/feedback/new", encoded_form.into()).await;
    let response = app.call(request).await.unwrap();
    let json: Feedback = parse_response_body(response).await;

    assert_eq!(json.title, form.title);
    assert_eq!(json.category, form.category);
    assert_eq!(json.status, form.status);
    assert_eq!(json.description, form.description);

    let edited_form = FeedbackForm {
        title: "Edited title".into(),
        category: "feature".into(),
        status: "in-progress".into(),
        description: "Another description".into(),
    };
    let encoded_edit_form = serde_urlencoded::to_string(&edited_form).unwrap();

    let request = patch_request(
        &format!("/feedback/{}/edit", json.id),
        encoded_edit_form.into(),
    )
    .await;
    let response = app.call(request).await.unwrap();
    let json: Feedback = parse_response_body(response).await;

    assert_eq!(json.title, edited_form.title);
    assert_eq!(json.category, edited_form.category);
    assert_eq!(json.status, edited_form.status);
    assert_eq!(json.description, edited_form.description);
}

