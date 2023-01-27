use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    image: String,
    name: String,
    username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    id: usize,
    title: String,
    category: String,
    upvotes: usize,
    upvoted: bool,
    status: String,
    description: String,
    comments: Option<Vec<Comment>>,
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
            comments: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    id: usize,
    content: String,
    replying_to: Option<String>,
    user: User,
    replies: Option<Vec<Comment>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppState {
    current_user: User,
    product_requests: Vec<Request>,
    // HACK: since there's no real database I'll just store these for convinence
    last_request_id: usize,
    last_comment_id: usize,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_user: User {
                image: "/image-zena.jpg".into(),
                name: "Zena Kelley".into(),
                username: "velvetround".into(),
            },
            product_requests: vec![
              Request {
                id: 1,
                title: "Add tags for solutions".into(),
                category: "enhancement".into(),
                upvotes: 112,
                upvoted: false,
                status: "suggestion".into(),
                description: "Easier to search for solutions based on a specific stack.".into(),
                comments: Some(vec![
                  Comment {
                    id: 1,
                    content: "Awesome idea! Trying to find framework-specific projects within the hubs can be tedious".into(),
                    user: User {
                      image: "/image-suzanne.jpg".into(),
                      name: "Suzanne Chang".into(),
                      username: "upbeat1811".into()
                    },
                    replying_to: None,
                    replies: None,
                  },
                  Comment {
                    id: 2,
                    content: "Please use fun, color-coded labels to easily identify them at a glance".into(),
                    user: User {
                      image: "/image-thomas.jpg".into(),
                      name: "Thomas Hood".into(),
                      username: "brawnybrave".into()
                    },
                    replying_to: None,
                    replies: None,
                  }
                ]),
                },
              Request {
                id: 2,
                title: "Add a dark theme option".into(),
                category: "feature".into(),
                upvotes: 99,
                upvoted: false,
                status: "suggestion".into(),
                description: "It would help people with light sensitivities and who prefer dark mode.".into(),
                comments: Some(vec![
                  Comment {
                    id: 3,
                    content: "Also, please allow styles to be applied based on system preferences. I would love to be able to browse Frontend Mentor in the evening after my device’s dark mode turns on without the bright background it currently has.".into(),
                    user:User  {
                      image: "/image-elijah.jpg".into(),
                      name: "Elijah Moss".into(),
                      username: "hexagon.bestagon".into()
                    },
                    replying_to: None,
                    replies: None
                  },
                  Comment {
                    id: 4,
                    content: "Second this! I do a lot of late night coding and reading. Adding a dark theme can be great for preventing eye strain and the headaches that result. It’s also quite a trend with modern apps and  apparently saves battery life.".into(),
                    user: User {
                      image: "/image-james.jpg".into(),
                      name: "James Skinner".into(),
                      username: "hummingbird1".into()
                    },
                    replying_to: None,
                    replies: Some(vec![
                      Comment {
                        id: 18,
                        content: "While waiting for dark mode, there are browser extensions that will also do the job. Search for 'dark theme' followed by your browser. There might be a need to turn off the extension for sites with naturally black backgrounds though.".into(),
                        replying_to: Some("hummingbird1".into()),
                        user: User {
                          image: "/image-anne.jpg".into(),
                          name: "Anne Valentine".into(),
                          username: "annev1990".into()
                        },
                        replies: None
                      },
                      Comment {
                        id: 19,
                        content: "Good point! Using any kind of style extension is great and can be highly customizable, like the ability to change contrast and brightness. I'd prefer not to use one of such extensions, however, for security and privacy reasons.".into(),
                        replying_to: Some("annev1990".into()),
                        user: User {
                          image: "/image-ryan.jpg".into(),
                          name: "Ryan Welles".into(),
                          username: "voyager.344".into()
                        },
                        replies: None
                      }
                    ])
                  }
                ])
              },
              Request {
                id: 3,
                title: "Q&A within the challenge hubs".into(),
                category: "feature".into(),
                upvotes: 65,
                upvoted: false,
                status: "suggestion".into(),
                description: "Challenge-specific Q&A would make for easy reference.".into(),
                comments: Some(vec![
               Comment    {
                    id: 5,
                    content: "Much easier to get answers from devs who can relate, since they've either finished the challenge themselves or are in the middle of it.".into(),
                    user:User  {
                      image: "/image-george.jpg".into(),
                      name: "George Partridge".into(),
                      username: "soccerviewer8".into()
                    },
                    replying_to: None,
                    replies: None
                  }
                ])
              },
              Request {
                id: 4,
                title: "Add image/video upload to feedback".into(),
                category: "enhancement".into(),
                upvotes: 51,
                upvoted: false,
                status: "suggestion".into(),
                description: "Images and screencasts can enhance comments on solutions.".into(),
                comments: Some(vec![
                  Comment {
                    id: 6,
                    content: "Right now, there is no ability to add images while giving feedback which isn't ideal because I have to use another app to show what I mean".into(),
                    user: User {
                      image: "/image-javier.jpg".into(),
                      name: "Javier Pollard".into(),
                      username: "warlikeduke".into()
                    },
                    replying_to: None,
                    replies: None
                  },
               Comment    {
                    id: 7,
                    content: "Yes I'd like to see this as well. Sometimes I want to add a short video or gif to explain the site's behavior..".into(),
                    user: User {
                      image: "/image-roxanne.jpg".into(),
                      name: "Roxanne Travis".into(),
                      username: "peppersprime32".into()
                    },
                    replying_to: None,
                    replies: None
                  }
                ])
              },
              Request {
                id: 5,
                title: "Ability to follow others".into(),
                category: "feature".into(),
                upvotes: 42,
                upvoted: false,
                status: "suggestion".into(),
                description: "Stay updated on comments and solutions other people post.".into(),
                comments: Some(vec![
                  Comment {
                    id: 8,
                    content: "I also want to be notified when devs I follow submit projects on FEM. Is in-app notification also in the pipeline?".into(),
                    user: User {
                      image: "/image-victoria.jpg".into(),
                      name: "Victoria Mejia".into(),
                      username: "arlen_the_marlin".into()
                    },
                    replying_to: None,
                    replies: Some(vec![
                   Comment    {
                        id: 17,
                        content: "Bumping this. It would be good to have a tab with a feed of people I follow so it's easy to see what challenges they’ve done lately. I learn a lot by reading good developers' code.".into(),
                        replying_to: Some("arlen_the_marlin".into()),
                        user: User {
                          image: "/image-zena.jpg".into(),
                          name: "Zena Kelley".into(),
                          username: "velvetround".into()
                        },
                        replies: None,
                      }
                    ])
                  },
               Comment {
                  id: 9,
                  content: "I've been saving the profile URLs of a few people and I check what they’ve been doing from time to time. Being able to follow them solves that".into(),
                  user: User {
                    image: "/image-jackson.jpg".into(),
                    name: "Jackson Barker".into(),
                    username: "countryspirit".into()
                  },
                  replying_to: None,
                  replies: None
                }
              ])
              },
              Request {
                id: 6,
                title: "Preview images not loading".into(),
                category: "bug".into(),
                upvotes: 3,
                upvoted: false,
                status: "suggestion".into(),
                description: "Challenge preview images are missing when you apply a filter.".into(),
                comments: None
              },
              Request {
                id: 7,
                title: "More comprehensive reports".into(),
                category: "feature".into(),
                upvotes: 123,
                upvoted: false,
                status: "planned".into(),
                description: "It would be great to see a more detailed breakdown of solutions.".into(),
                comments: Some(vec![
                  Comment {
                    id: 10,
                    content: "This would be awesome! It would be so helpful to see an overview of my code in a way that makes it easy to spot where things could be improved.".into(),
                    user: User {
                      image: "/image-victoria.jpg".into(),
                      name: "Victoria Mejia".into(),
                      username: "arlen_the_marlin".into()
                    },
                    replying_to: None,
                    replies: None
                  },
                Comment {
                  id: 11,
                  content: "Yeah, this would be really good. I'd love to see deeper insights into my code!".into(),
                  user: User {
                    image: "/image-jackson.jpg".into(),
                    name: "Jackson Barker".into(),
                    username: "countryspirit".into()
                  },
                  replying_to: None,
                  replies: None
                }
              ])
              },
              Request {
                id: 8,
                title: "Learning paths".into(),
                category: "feature".into(),
                upvotes: 28,
                upvoted: false,
                status: "planned".into(),
                description: "Sequenced projects for different goals to help people improve.".into(),
                comments: Some(vec![
               Comment    {
                    id: 12,
                    content: "Having a path through the challenges that I could follow would be brilliant! Sometimes I'm not sure which challenge would be the best next step to take. So this would help me navigate through them!".into(),
                    user: User {
                      image: "/image-george.jpg".into(),
                      name: "George Partridge".into(),
                      username: "soccerviewer8".into()
                    },
                    replying_to: None,
                    replies: None
                  }
                ])
              },
              Request {
                id: 9,
                title: "One-click portfolio generation".into(),
                category: "feature".into(),
                upvotes: 62,
                upvoted: false,
                status: "in-progress".into(),
                description: "Add ability to create professional looking portfolio from profile.".into(),
                comments: Some(vec![
               Comment    {
                    id: 13,
                    content: "I haven't built a portfolio site yet, so this would be really helpful. Might it also be possible to choose layout and colour themes?!".into(),
                    user: User {
                      image: "/image-ryan.jpg".into(),
                      name: "Ryan Welles".into(),
                      username: "voyager.344".into()
                    },
                    replying_to: None,
                    replies: None
                  }
                ])
              },
              Request {
                id: 10,
                title: "Bookmark challenges".into(),
                category: "feature".into(),
                upvotes: 31,
                upvoted: false,
                status: "in-progress".into(),
                description: "Be able to bookmark challenges to take later on.".into(),
                comments: Some(vec![
                  Comment {
                    id: 14,
                    content: "This would be great! At the moment, I'm just starting challenges in order to save them. But this means the My Challenges section is overflowing with projects and is hard to manage. Being able to bookmark challenges would be really helpful.".into(),
                    user: User {
                      image: "/image-suzanne.jpg".into(),
                      name: "Suzanne Chang".into(),
                      username: "upbeat1811".into()
                    },
                    replying_to: None,
                    replies: None
                  }
                ])
              },
              Request {
                id: 11,
                title: "Animated solution screenshots".into(),
                category: "bug".into(),
                upvotes: 9,
                upvoted: false,
                status: "in-progress".into(),
                description: "Screenshots of solutions with animations don’t display correctly.".into(),
                comments: None
              },
              Request {
                id: 12,
                title: "Add micro-interactions".into(),
                category: "enhancement".into(),
                upvotes: 71,
                upvoted: false,
                status: "live".into(),
                description: "Small animations at specific points can add delight.".into(),
                comments: Some(vec![
                  Comment {
                    id: 15,
                    content: "I'd love to see this! It always makes me so happy to see little details like these on websites.".into(),
                    user: User {
                      image: "/image-victoria.jpg".into(),
                      name: "Victoria Mejia".into(),
                      username: "arlen_the_marlin".into()
                    },
                    replying_to: None,
                    replies: Some(vec![
                     Comment {
                        id: 16,
                        content: "Me too! I'd also love to see celebrations at specific points as well. It would help people take a moment to celebrate their achievements!".into(),
                        replying_to: Some("arlen_the_marlin".into()),
                        replies: None,
                        user: User {
                          image: "/image-suzanne.jpg".into(),
                          name: "Suzanne Chang".into(),
                          username: "upbeat1811".into()
                        }
                      }
                    ])
                  }
                ])
              }
            ],
              last_request_id: 12,
              last_comment_id: 13
        }
    }

    pub fn next_request_id(&mut self) -> usize {
        self.last_request_id += 1;
        self.last_request_id
    }

    pub fn next_comment_id(&mut self) -> usize {
        self.last_comment_id += 1;
        self.last_comment_id
    }

    pub fn new_request(&mut self, title: String, category: String, description: String) -> Request {
        let request = Request::new(self.next_request_id(), title, category, description);
        self.product_requests.push(request.clone());

        request
    }

    pub fn edit_request(
        &mut self,
        id: usize,
        title: String,
        category: String,
        description: String,
    ) -> Result<(), String> {
        let request = self
            .product_requests
            .iter_mut()
            .find(|req| req.id == id)
            .ok_or(String::from(format!("could not find request with id {id}")))?;

        request.title = title;
        request.category = category;
        request.description = description;

        Ok(())
    }
}
