fn main() {
    let mut post = Post::new();

    post.add_text("heyy ;)");
    println!("add_text: {}", post.content());

    post.request_review();
    println!("request_review: {}", post.content());

    post.add_text("asdfasdfasdf");

    post.approve();
    println!("approve: {}", post.content());

    post.reject();
    println!("reject: {}", post.content());

    post.request_review();
    println!("request_review: {}", post.content());

    post.approve();
    println!("approve: {}", post.content());

    post.approve();
    println!("approve: {}", post.content());

    post.reject();
    println!("reject: {}", post.content());
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if let Some(s) = self.state.take() {
            s.add_text(self, text);
            self.state = Some(s);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn add_text(&self, _post: &mut Post, _text: &str) {}

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn add_text(&self, post: &mut Post, text: &str) {
        post.content.push_str(text);
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::default())
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[derive(Default)]
struct PendingReview {
    num_approvals: u32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.num_approvals += 1;

        if self.num_approvals >= 2 {
            Box::new(Published {})
        } else {
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
