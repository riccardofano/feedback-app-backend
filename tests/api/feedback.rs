use crate::{
    helpers::{
        create_app, get_request, is_application_json, parse_response_body, patch_request,
        post_request,
    },
    schema::{Comment, CommentForm, Feedback, FeedbackForm, FeedbackWithComments, UpvoteUpdate},
};
use axum::{body::Body, http::StatusCode, Router};
use hyper::Response;
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

async fn create_feedback_response(
    app: &mut Router,
    form: &FeedbackForm,
) -> Response<http_body::combinators::UnsyncBoxBody<axum::body::Bytes, axum::Error>> {
    let encoded_form = serde_urlencoded::to_string(form).unwrap();
    let request = post_request("/feedback/new", encoded_form.into()).await;
    app.call(request).await.unwrap()
}

async fn create_base_comment(
    app: &mut Router,
    form: &CommentForm,
    id_feedback: i32,
) -> Response<http_body::combinators::UnsyncBoxBody<axum::body::Bytes, axum::Error>> {
    let encoded_form = serde_urlencoded::to_string(form).unwrap();
    let request = post_request(
        &format!("/feedback/{}/comment", id_feedback),
        encoded_form.into(),
    )
    .await;
    app.call(request).await.unwrap()
}

#[tokio::test]
async fn post_new_feedback() {
    let mut app = create_app().await;

    let form = FeedbackForm::default();
    let response = create_feedback_response(&mut app, &form).await;

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

    let form = FeedbackForm::default();
    let response = create_feedback_response(&mut app, &form).await;
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

#[tokio::test]
async fn upvote_unupvote_feedback() {
    let mut app = create_app().await;

    let form = FeedbackForm::default();
    let response = create_feedback_response(&mut app, &form).await;
    let json: Feedback = parse_response_body(response).await;

    assert!(!json.upvoted);
    assert_eq!(json.upvotes, 0);

    let upvote_request =
        post_request(&format!("/feedback/{}/upvote", json.id), Body::empty()).await;
    let upvoted_json: UpvoteUpdate =
        parse_response_body(app.call(upvote_request).await.unwrap()).await;

    assert!(upvoted_json.upvoted);
    assert_eq!(upvoted_json.upvotes, json.upvotes + 1);

    let unupvote_request =
        post_request(&format!("/feedback/{}/upvote", json.id), Body::empty()).await;
    let unupvoted_json: UpvoteUpdate =
        parse_response_body(app.call(unupvote_request).await.unwrap()).await;

    assert!(!unupvoted_json.upvoted);
    assert_eq!(unupvoted_json.upvotes, json.upvotes);
    assert_eq!(unupvoted_json.upvotes, upvoted_json.upvotes - 1);
}

#[tokio::test]
async fn post_feedback_comment() {
    let mut app = create_app().await;

    let form = FeedbackForm::default();
    let response = create_feedback_response(&mut app, &form).await;
    let json: Feedback = parse_response_body(response).await;

    // NOTE: usernames are seeded at the momement and I'm using a known one.
    let comment = CommentForm {
        username: "velvetround".into(),
        content: "A great comment".into(),
    };
    let encoded_comment = serde_urlencoded::to_string(&comment).unwrap();
    let request = post_request(
        &format!("/feedback/{}/comment", json.id),
        encoded_comment.into(),
    )
    .await;
    let response = app.call(request).await.unwrap();
    let json_comment: Comment = parse_response_body(response).await;

    assert_eq!(json_comment.id_request, json.id);
    assert_eq!(json_comment.id_parent, None);
    assert_eq!(json_comment.user.username, comment.username);
    assert_eq!(json_comment.content, comment.content);

    let request = get_request(&format!("/feedback/{}", json.id), Body::empty()).await;
    let response = app.call(request).await.unwrap();
    let feedback_json: FeedbackWithComments = parse_response_body(response).await;

    assert_eq!(feedback_json.feedback.id, json.id);
    assert_eq!(feedback_json.comments.len(), 1);
    assert_eq!(feedback_json.comments[0].id, json_comment.id);
    assert_eq!(
        feedback_json.comments[0].user.username,
        json_comment.user.username
    );
    assert_eq!(feedback_json.comments[0].content, json_comment.content);
    assert!(feedback_json.comments[0].replies.is_empty());
}

#[tokio::test]
async fn post_feedback_comment_with_missing_fields() {
    let mut app = create_app().await;

    let form = FeedbackForm::default();
    let response = create_feedback_response(&mut app, &form).await;
    let json: Feedback = parse_response_body(response).await;

    let uri = &format!("/feedback/{}/comment", json.id);

    let encoded_comment = String::from("content=Some%20comment");
    let request = post_request(uri, encoded_comment.into()).await;
    let response = app.call(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "expected to be missing username/owner"
    );

    let encoded_comment = String::from("username=velvetround");
    let request = post_request(uri, encoded_comment.into()).await;
    let response = app.call(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "expected to be missing content"
    );

    let request = get_request(&format!("/feedback/{}", json.id), Body::empty()).await;
    let response = app.call(request).await.unwrap();
    let feedback: FeedbackWithComments = parse_response_body(response).await;

    assert_eq!(feedback.feedback.id, json.id);
    assert!(feedback.comments.is_empty())
}

#[tokio::test]
async fn post_feedback_reply() {
    let mut app = create_app().await;

    let form = FeedbackForm::default();
    let response = create_feedback_response(&mut app, &form).await;
    let json: Feedback = parse_response_body(response).await;

    let encoded_comment = serde_urlencoded::to_string(CommentForm {
        username: "velvetround".into(),
        content: "Some content".into(),
    })
    .unwrap();
    let commment_request = post_request(
        &format!("/feedback/{}/comment", json.id),
        encoded_comment.into(),
    )
    .await;
    let response = app.call(commment_request).await.unwrap();
    let comment_json: Comment = parse_response_body(response).await;

    let reply = CommentForm {
        username: "soccerviewer8".into(),
        content: "A thoroughly thought out reply".into(),
    };
    let encoded_reply = serde_urlencoded::to_string(&reply).unwrap();
    let uri = &format!("/feedback/{}/comment/{}/reply", json.id, comment_json.id);
    let reply_request = post_request(uri, encoded_reply.into()).await;
    let response = app.call(reply_request).await.unwrap();

    let reply_json: Comment = parse_response_body(response).await;

    assert_eq!(reply_json.id_request, json.id);
    assert_eq!(reply_json.id_parent, Some(comment_json.id));
    assert_eq!(reply_json.content, reply.content);

    let feedback_request = get_request(&format!("/feedback/{}", json.id), Body::empty()).await;
    let response = app.call(feedback_request).await.unwrap();

    let complete_feedback: FeedbackWithComments = parse_response_body(response).await;

    assert_eq!(complete_feedback.feedback.id, json.id);
    assert_eq!(complete_feedback.comments.len(), 1);
    assert_eq!(complete_feedback.comments[0].id, comment_json.id);
    assert_eq!(complete_feedback.comments[0].replies.len(), 1);
    assert_eq!(complete_feedback.comments[0].replies[0].id, reply_json.id);
    assert_eq!(
        complete_feedback.comments[0].replies[0].content,
        reply_json.content
    );
}

#[tokio::test]
async fn post_feedback_reply_nested() {
    let mut app = create_app().await;

    let form = FeedbackForm::default();
    let response = create_feedback_response(&mut app, &form).await;
    let json: Feedback = parse_response_body(response).await;

    let encoded_comment = serde_urlencoded::to_string(CommentForm::default()).unwrap();
    let commment_request = post_request(
        &format!("/feedback/{}/comment", json.id),
        encoded_comment.into(),
    )
    .await;
    let comment_json: Comment =
        parse_response_body(app.call(commment_request).await.unwrap()).await;

    let reply1 = serde_urlencoded::to_string(CommentForm::default()).unwrap();
    let uri = &format!("/feedback/{}/comment/{}/reply", json.id, comment_json.id);
    let response = app
        .call(post_request(uri, reply1.into()).await)
        .await
        .unwrap();
    let reply1: Comment = parse_response_body(response).await;

    let reply2 = serde_urlencoded::to_string(CommentForm::default()).unwrap();
    let uri = &format!("/feedback/{}/comment/{}/reply", json.id, reply1.id);
    let response = app
        .call(post_request(uri, reply2.into()).await)
        .await
        .unwrap();
    let reply2: Comment = parse_response_body(response).await;

    let reply3 = serde_urlencoded::to_string(CommentForm::default()).unwrap();
    let uri = &format!("/feedback/{}/comment/{}/reply", json.id, reply2.id);
    let response = app
        .call(post_request(uri, reply3.into()).await)
        .await
        .unwrap();
    let reply3: Comment = parse_response_body(response).await;

    let feedback_request = get_request(&format!("/feedback/{}", json.id), Body::empty()).await;
    let response = app.call(feedback_request).await.unwrap();

    let complete_feedback: FeedbackWithComments = parse_response_body(response).await;

    assert_eq!(complete_feedback.feedback.id, json.id);
    assert_eq!(complete_feedback.comments.len(), 1);
    assert_eq!(complete_feedback.comments[0].id, comment_json.id);
    assert_eq!(complete_feedback.comments[0].replies.len(), 1);
    assert_eq!(complete_feedback.comments[0].replies[0].id, reply1.id);
    assert_eq!(complete_feedback.comments[0].replies[0].replies.len(), 1);
    #[rustfmt::skip]
    assert_eq!(complete_feedback.comments[0].replies[0].replies[0].id, reply2.id);
    #[rustfmt::skip]
    assert_eq!(complete_feedback.comments[0].replies[0].replies[0] .replies .len(), 1);
    #[rustfmt::skip]
    assert_eq!(complete_feedback.comments[0].replies[0].replies[0].replies[0].id, reply3.id);
}

#[tokio::test]
async fn comment_has_user_information() {
    let mut app = create_app().await;

    let form = FeedbackForm::default();
    let response = create_feedback_response(&mut app, &form).await;
    let json: Feedback = parse_response_body(response).await;

    let comment = CommentForm {
        username: "velvetround".into(),
        content: "Whatever".into(),
    };
    let response = create_base_comment(&mut app, &comment, json.id).await;
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let string = String::from_utf8(body.to_vec()).unwrap();

    dbg!(&string);

    // NOTE: I'm searching the string because otherwise I'd have to modify the
    // Comment struct first before writing a failing test
    assert!(string.contains(r#""user":{"#));
    assert!(string.contains(r#""image":"/image-zena.jpg""#));
    assert!(string.contains(r#""name":"Zena Kelley""#));
    assert!(string.contains(r#""username":"velvetround""#));
}
