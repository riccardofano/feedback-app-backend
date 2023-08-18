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
pub struct FeedbackForm {
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

