use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    id: u32,
    title: String,
    category: String,
    upvotes: u32,
    status: String,
    description: String,
    comments: Option<String>,
}

impl Request {
    pub fn new(title: String, category: String, description: String) -> Self {
        Self {
            id: 1,
            title,
            category,
            upvotes: 0,
            status: String::from("suggestion"),
            description,
            comments: None,
        }
    }
}
