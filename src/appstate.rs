use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    username: String,
    name: String,
    image: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    id: usize,
    title: String,
    category: String,
    upvotes: isize,
    upvoted: bool,
    status: String,
    description: String,
    comments: Vec<usize>,
}

impl Request {
    fn new(id: usize, title: String, category: String, description: String) -> Self {
        Self {
            id,
            title,
            category,
            upvotes: 0,
            upvoted: false,
            status: "suggestion".into(),
            description,
            comments: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    id: usize,
    content: String,
    replying_to: Option<String>,
    user: String,
    replies: Vec<usize>,
}

impl Comment {
    pub fn new(id: usize, user: String, content: String, to: Option<String>) -> Self {
        Self {
            id,
            user,
            content,
            replying_to: to,
            replies: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ComposedRequest {
    id: usize,
    pub title: String,
    pub category: String,
    pub upvotes: isize,
    pub upvoted: bool,
    pub status: String,
    pub description: String,
    pub comments: Vec<ComposedComment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ComposedComment {
    id: usize,
    pub content: String,
    pub replying_to: Option<String>,
    pub user: User,
    pub replies: Vec<ComposedComment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub current_user: String,
    pub product_requests: HashMap<usize, Request>,
    pub users: HashMap<String, User>,
    pub comments: HashMap<usize, Comment>,
    last_request_id: usize,
    last_comment_id: usize,
}

impl AppState {
    pub fn seeded() -> Self {
        let users = HashMap::from([
            (
                "brawnybrave".into(),
                User {
                    image: "/image-thomas.jpg".into(),
                    name: "Thomas Hood".into(),
                    username: "brawnybrave".into(),
                },
            ),
            (
                "hexagon.bestagon".into(),
                User {
                    image: "/image-elijah.jpg".into(),
                    name: "Elijah Moss".into(),
                    username: "hexagon.bestagon".into(),
                },
            ),
            (
                "annev1990".into(),
                User {
                    image: "/image-anne.jpg".into(),
                    name: "Anne Valentine".into(),
                    username: "annev1990".into(),
                },
            ),
            (
                "warlikeduke".into(),
                User {
                    image: "/image-javier.jpg".into(),
                    name: "Javier Pollard".into(),
                    username: "warlikeduke".into(),
                },
            ),
            (
                "peppersprime32".into(),
                User {
                    image: "/image-roxanne.jpg".into(),
                    name: "Roxanne Travis".into(),
                    username: "peppersprime32".into(),
                },
            ),
            (
                "velvetround".into(),
                User {
                    image: "/image-zena.jpg".into(),
                    name: "Zena Kelley".into(),
                    username: "velvetround".into(),
                },
            ),
            (
                "countryspirit".into(),
                User {
                    image: "/image-jackson.jpg".into(),
                    name: "Jackson Barker".into(),
                    username: "countryspirit".into(),
                },
            ),
            (
                "soccerviewer8".into(),
                User {
                    image: "/image-george.jpg".into(),
                    name: "George Partridge".into(),
                    username: "soccerviewer8".into(),
                },
            ),
            (
                "voyager.344".into(),
                User {
                    image: "/image-ryan.jpg".into(),
                    name: "Ryan Welles".into(),
                    username: "voyager.344".into(),
                },
            ),
            (
                "upbeat1811".into(),
                User {
                    image: "/image-suzanne.jpg".into(),
                    name: "Suzanne Chang".into(),
                    username: "upbeat1811".into(),
                },
            ),
            (
                "arlen_the_marlin".into(),
                User {
                    image: "/image-victoria.jpg".into(),
                    name: "Victoria Mejia".into(),
                    username: "arlen_the_marlin".into(),
                },
            ),
            (
                "hummingbird1".into(),
                User {
                    image: "/image-james.jpg".into(),
                    name: "James Skinner".into(),
                    username: "hummingbird1".into(),
                },
            ),
        ]);

        let comments = HashMap::from([
            (1,
            Comment {
                id: 1,
                content: "Awesome idea! Trying to find framework-specific projects within the hubs can be tedious".into(),
                user: "upbeat1811".into(),
                replying_to: None,
                replies: Vec::new(),
            },
            ),
            (2,
            Comment {
                id: 2,
                content: "Please use fun, color-coded labels to easily identify them at a glance".into(),
                user: "brawnybrave".into(),
                replying_to: None,
                replies: Vec::new(),
            },
            ),
            (3,
            Comment {
                id: 3,
                content: "Also, please allow styles to be applied based on system preferences. I would love to be able to browse Frontend Mentor in the evening after my device’s dark mode turns on without the bright background it currently has.".into(),
                user: "hexagon.bestagon".into(),
                replying_to: None,
                replies: Vec::new()
            },
            ),
            (4,
            Comment {
                id: 4,
                content: "Second this! I do a lot of late night coding and reading. Adding a dark theme can be great for preventing eye strain and the headaches that result. It’s also quite a trend with modern apps and  apparently saves battery life.".into(),
                user: "hummingbird1".into(),
                replying_to: None,
                replies: vec![18, 19]
            },
            ),
            (5,
            Comment {
                id: 5,
                content: "Much easier to get answers from devs who can relate, since they've either finished the challenge themselves or are in the middle of it.".into(),
                user: "soccerviewer8".into(),
                replying_to: None,
                replies: Vec::new()
            },
            ),
            (6,
            Comment {
                id: 6,
                content: "Right now, there is no ability to add images while giving feedback which isn't ideal because I have to use another app to show what I mean".into(),
                user: "warlikeduke".into(),
                replying_to: None,
                replies: Vec::new()
            },
            ),
            (7,
            Comment {
                id: 7,
                content: "Yes I'd like to see this as well. Sometimes I want to add a short video or gif to explain the site's behavior..".into(),
                user: "peppersprime32".into(),
                replying_to: None,
                replies: Vec::new()
            },
            ),
            (8,
            Comment {
                id: 8,
                content: "I also want to be notified when devs I follow submit projects on FEM. Is in-app notification also in the pipeline?".into(),
                user: "arlen_the_marlin".into(),
                replying_to: None,
                replies: vec![17]
            },
            ),
            (9,
            Comment {
                id: 9,
                content: "I've been saving the profile URLs of a few people and I check what they’ve been doing from time to time. Being able to follow them solves that".into(),
                user: "countryspirit".into(),
                replying_to: None,
                replies: Vec::new()
            },
            ),
            (10,
            Comment {
                id: 10,
                content: "This would be awesome! It would be so helpful to see an overview of my code in a way that makes it easy to spot where things could be improved.".into(),
                user: "arlen_the_marlin".into(),
                replying_to: None,
                replies: Vec::new()
            },
            ),
            (11,
            Comment {
                id: 11,
                content: "Yeah, this would be really good. I'd love to see deeper insights into my code!".into(),
                user: "countryspirit".into(),
                replying_to: None,
                replies: Vec::new()
            },
            ),
            (12,
            Comment {
                id: 12,
                content: "Having a path through the challenges that I could follow would be brilliant! Sometimes I'm not sure which challenge would be the best next step to take. So this would help me navigate through them!".into(),
                user: "soccerviewer8".into(),
                replying_to: None,
                replies: Vec::new()
            },
            ),
            (13,
            Comment {
                id: 13,
                content: "I haven't built a portfolio site yet, so this would be really helpful. Might it also be possible to choose layout and colour themes?!".into(),
                user: "voyager.344".into(),
                replying_to: None,
                replies: Vec::new()
            },
            ),
            (14,
            Comment {
                id: 14,
                content: "This would be great! At the moment, I'm just starting challenges in order to save them. But this means the My Challenges section is overflowing with projects and is hard to manage. Being able to bookmark challenges would be really helpful.".into(),
                user: "upbeat1811".into(),
                replying_to: None,
                replies: Vec::new()
            },
            ),
            (15,
            Comment {
                id: 15,
                content: "I'd love to see this! It always makes me so happy to see little details like these on websites.".into(),
                user: "arlen_the_marlin".into(),
                replying_to: None,
                replies: vec![16]
            },
            ),
            (16,
            Comment {
                id: 16,
                content: "Me too! I'd also love to see celebrations at specific points as well. It would help people take a moment to celebrate their achievements!".into(),
                replying_to: Some("arlen_the_marlin".into()),
                replies: Vec::new(),
                user: "upbeat1811".into()
            },
            ),
            (17,
            Comment {
                id: 17,
                content: "Bumping this. It would be good to have a tab with a feed of people I follow so it's easy to see what challenges they’ve done lately. I learn a lot by reading good developers' code.".into(),
                replying_to: Some("arlen_the_marlin".into()),
                user: "velvetround".into(),
                replies: Vec::new(),
            },
            ),
            (18,
            Comment {
                id: 18,
                content: "While waiting for dark mode, there are browser extensions that will also do the job. Search for 'dark theme' followed by your browser. There might be a need to turn off the extension for sites with naturally black backgrounds though.".into(),
                replying_to: Some("hummingbird1".into()),
                user: "annev1990".into(),
                replies: Vec::new()
            },
            ),
            (19,
            Comment {
                id: 19,
                content: "Good point! Using any kind of style extension is great and can be highly customizable, like the ability to change contrast and brightness. I'd prefer not to use one of such extensions, however, for security and privacy reasons.".into(),
                replying_to: Some("annev1990".into()),
                user: "voyager.344".into(),
                replies: Vec::new()
            },
            )
        ]);

        let product_requests = HashMap::from([
            (
                1,
                Request {
                    id: 1,
                    title: "Add tags for solutions".into(),
                    category: "enhancement".into(),
                    upvotes: 112,
                    upvoted: false,
                    status: "suggestion".into(),
                    description: "Easier to search for solutions based on a specific stack.".into(),
                    comments: vec![1, 2],
                },
            ),
            (
                2,
                Request {
                    id: 2,
                    title: "Add a dark theme option".into(),
                    category: "feature".into(),
                    upvotes: 99,
                    upvoted: false,
                    status: "suggestion".into(),
                    description:
                        "It would help people with light sensitivities and who prefer dark mode."
                            .into(),
                    comments: vec![3, 4],
                },
            ),
            (
                3,
                Request {
                    id: 3,
                    title: "Q&A within the challenge hubs".into(),
                    category: "feature".into(),
                    upvotes: 65,
                    upvoted: false,
                    status: "suggestion".into(),
                    description: "Challenge-specific Q&A would make for easy reference.".into(),
                    comments: vec![5],
                },
            ),
            (
                4,
                Request {
                    id: 4,
                    title: "Add image/video upload to feedback".into(),
                    category: "enhancement".into(),
                    upvotes: 51,
                    upvoted: false,
                    status: "suggestion".into(),
                    description: "Images and screencasts can enhance comments on solutions.".into(),
                    comments: vec![6, 7],
                },
            ),
            (
                5,
                Request {
                    id: 5,
                    title: "Ability to follow others".into(),
                    category: "feature".into(),
                    upvotes: 42,
                    upvoted: false,
                    status: "suggestion".into(),
                    description: "Stay updated on comments and solutions other people post.".into(),
                    comments: vec![9],
                },
            ),
            (
                6,
                Request {
                    id: 6,
                    title: "Preview images not loading".into(),
                    category: "bug".into(),
                    upvotes: 3,
                    upvoted: false,
                    status: "suggestion".into(),
                    description: "Challenge preview images are missing when you apply a filter."
                        .into(),
                    comments: Vec::new(),
                },
            ),
            (
                7,
                Request {
                    id: 7,
                    title: "More comprehensive reports".into(),
                    category: "feature".into(),
                    upvotes: 123,
                    upvoted: false,
                    status: "planned".into(),
                    description: "It would be great to see a more detailed breakdown of solutions."
                        .into(),
                    comments: vec![10, 11],
                },
            ),
            (
                8,
                Request {
                    id: 8,
                    title: "Learning paths".into(),
                    category: "feature".into(),
                    upvotes: 28,
                    upvoted: false,
                    status: "planned".into(),
                    description: "Sequenced projects for different goals to help people improve."
                        .into(),
                    comments: vec![12],
                },
            ),
            (
                9,
                Request {
                    id: 9,
                    title: "One-click portfolio generation".into(),
                    category: "feature".into(),
                    upvotes: 62,
                    upvoted: false,
                    status: "in-progress".into(),
                    description:
                        "Add ability to create professional looking portfolio from profile.".into(),
                    comments: vec![13],
                },
            ),
            (
                10,
                Request {
                    id: 10,
                    title: "Bookmark challenges".into(),
                    category: "feature".into(),
                    upvotes: 31,
                    upvoted: false,
                    status: "in-progress".into(),
                    description: "Be able to bookmark challenges to take later on.".into(),
                    comments: vec![14],
                },
            ),
            (
                11,
                Request {
                    id: 11,
                    title: "Animated solution screenshots".into(),
                    category: "bug".into(),
                    upvotes: 9,
                    upvoted: false,
                    status: "in-progress".into(),
                    description:
                        "Screenshots of solutions with animations don’t display correctly.".into(),
                    comments: Vec::new(),
                },
            ),
            (
                12,
                Request {
                    id: 12,
                    title: "Add micro-interactions".into(),
                    category: "enhancement".into(),
                    upvotes: 71,
                    upvoted: false,
                    status: "live".into(),
                    description: "Small animations at specific points can add delight.".into(),
                    comments: vec![15],
                },
            ),
        ]);

        let last_request_id = &product_requests.len();
        let last_comment_id = &comments.len();

        Self {
            current_user: "velvetround".into(),
            product_requests,
            users,
            comments,
            last_request_id: *last_request_id,
            last_comment_id: *last_comment_id,
        }
    }

    fn next_request_id(&mut self) -> usize {
        self.last_request_id += 1;
        self.last_request_id
    }

    fn _next_comment_id(&mut self) -> usize {
        self.last_comment_id += 1;
        self.last_comment_id
    }

    pub fn new_request(&mut self, title: String, category: String, description: String) -> Request {
        let request = Request::new(self.next_request_id(), title, category, description);
        self.product_requests.insert(request.id, request.clone());

        request
    }

    fn get_request_by_id(&self, id: usize) -> Request {
        self.product_requests.get(&id).unwrap().clone()
    }

    fn get_mut_request_by_id(&mut self, id: usize) -> &mut Request {
        self.product_requests.get_mut(&id).unwrap()
    }

    pub fn get_request(&self, id: usize) -> Result<ComposedRequest, String> {
        let request = self.get_request_by_id(id);
        let request = self.compose_request(request);

        Ok(request)
    }

    pub fn upvote_request(&mut self, id: usize) -> Result<ComposedRequest, String> {
        let mut request = self
            .product_requests
            .get_mut(&id)
            .ok_or(format!("could not find request with id {id}"))?;

        request.upvotes += if request.upvoted { -1 } else { 1 };
        request.upvoted = !request.upvoted;

        let request = request.clone();
        let composed = self.compose_request(request);

        Ok(composed)
    }

    pub fn edit_request(
        &mut self,
        id: usize,
        title: String,
        category: String,
        status: String,
        description: String,
    ) -> Result<ComposedRequest, String> {
        let mut request = self
            .product_requests
            .get_mut(&id)
            .ok_or(format!("could not find request with id {id}"))?;

        request.title = title;
        request.category = category;
        request.status = status;
        request.description = description;

        let request = request.clone();
        let composed = self.compose_request(request);

        Ok(composed)
    }

    pub fn new_comment(
        &mut self,
        request_id: usize,
        username: String,
        content: String,
    ) -> ComposedComment {
        let comment = Comment::new(self._next_comment_id(), username, content, None);
        let request = self.get_mut_request_by_id(request_id);

        request.comments.push(comment.id);
        self.comments.insert(comment.id, comment.clone());

        self.compose_replies(&comment)
    }

    pub fn new_reply(
        &mut self,
        comment_id: usize,
        username: String,
        content: String,
        to: String,
    ) -> ComposedComment {
        let reply = Comment::new(self._next_comment_id(), username, content, Some(to));
        let comment = self.comments.get_mut(&comment_id).unwrap();

        comment.replies.push(reply.id);
        self.comments.insert(reply.id, reply.clone());

        self.compose_replies(&reply)
    }

    pub fn get_current_user(&self) -> &User {
        self.users.get(&self.current_user).unwrap()
    }

    pub fn get_all_requests(&self) -> Vec<ComposedRequest> {
        self.product_requests
            .values()
            .map(|req| self.compose_request(req.clone()))
            .collect()
    }

    fn compose_request(&self, request: Request) -> ComposedRequest {
        let mut comments = vec![];
        for comment in request.comments.iter() {
            if let Some(found) = self.comments.get(comment) {
                comments.push(self.compose_replies(&found))
            }
        }

        ComposedRequest {
            id: request.id,
            title: request.title.clone(),
            category: request.category.clone(),
            upvotes: request.upvotes,
            upvoted: request.upvoted,
            status: request.status.clone(),
            description: request.description.clone(),
            comments,
        }
    }

    fn compose_replies(&self, comment: &Comment) -> ComposedComment {
        let mut replies = vec![];
        for reply in comment.replies.iter() {
            if let Some(found) = self.comments.get(reply) {
                replies.push(self.compose_replies(found))
            }
        }

        // TODO: return a 400 if username is not found
        let user = self.users.get(&comment.user).unwrap().clone();

        ComposedComment {
            id: comment.id,
            content: comment.content.clone(),
            replying_to: comment.replying_to.clone(),
            user,
            replies,
        }
    }
}
