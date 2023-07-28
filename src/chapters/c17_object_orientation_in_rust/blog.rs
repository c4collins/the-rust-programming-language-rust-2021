pub struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}

impl Post {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            state: Some(Box::new(Draft {})),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_be_updated() {
            self.content.push_str(text);
        } else {
            println!("Unable to edit text in this state");
        }
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}

trait State {
    fn can_be_updated(&self) -> bool {
        false // Default to not allowing the text to be mutated
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}
impl State for Draft {
    // Allow updating the text in the Draft state
    fn can_be_updated(&self) -> bool {
        true
    }
    // Change to Pending Review
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    // Can't approve what's not PendingApproval
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Can't reject what's not PendingApproval
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    // We've already requested a review
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Convert Pending Review to Published
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingSecondApproval {})
    }
    // Convert PendingReview to Draft
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct PendingSecondApproval {}
impl State for PendingSecondApproval {
    // We've already requested a review
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Convert PendingSecondApproval to Published
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    // Revert the first Approval
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct Published {}
impl State for Published {
    // No need to review a Published Post
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // This Post is already Approved
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // This Post is already Approved
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Override the default
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
