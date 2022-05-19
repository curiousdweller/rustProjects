use std::rc::Rc;
struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft{})),
            /*
            Here when state is instansiated the box represents a fat pointer
                1. A pointer to the concrete Draft type Object
                2. A pointer to a Vtable , Examplar V table
            Some Notes on Virtual Tables:
           - There is one for each type.
           - Generated Statically at Compile Time.     

            
            */
            content : String::new()
        }
    }

    pub fn add_text(&mut self, text: &str) -> String {
        self.content.push_str(text);
        String::new()
        
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
        self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(S) = self.state.take() {
            self.state = Some(S.approve());
        }
    }

    pub fn content(&self) {
        let s = self.state.as_ref().unwrap();
        s.content(&self);
    }
}


trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}


struct Draft {

}

// Self is Draft. Ref to Self is Ref to Draft
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Pending{})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Pending {}

impl State for Pending {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        (post.content).as_str()
    }
}


#[cfg(test)]
mod tests {
    use crate::{Post, State};
    #[test]
    fn it_works() {
        let mut post = Post::new();
        if let Some(s) = post.state.take() {
            assert_eq!(s.content(&post), "");
        }
        post.add_text("I am Some Content");
        post.request_review();
        post.approve();
        assert_eq!(post.content, String::from("I am Some Content"));

    }
}


