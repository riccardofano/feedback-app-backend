use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Feedback {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub upvotes: i32,
    pub upvoted: bool,
    pub status: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeedbackWithComments {
    #[serde(flatten)]
    pub feedback: Feedback,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewFeedbackForm {
    pub title: String,
    pub category: String,
    pub description: String,
}

impl Default for NewFeedbackForm {
    fn default() -> Self {
        Self {
            title: String::from("New feedback"),
            category: String::from("bug"),
            description: String::from("Some description"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EditFeedbackForm {
    pub title: String,
    pub category: String,
    pub status: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpvoteUpdate {
    pub upvoted: bool,
    pub upvotes: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: i32,
    pub id_request: i32,
    pub id_parent: Option<i32>,
    pub user: User,
    pub content: String,
    pub replying_to: Option<String>,
    pub replies: Vec<Comment>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub username: String,
    pub image: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentForm {
    pub username: String,
    pub content: String,
}

impl Default for CommentForm {
    fn default() -> Self {
        Self {
            username: "velvetround".into(),
            content: "Some content".into(),
        }
    }
}
