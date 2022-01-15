pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            approval_count: 0,
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    approval_count: u32,
    content: String,
}

impl PendingReviewPost {
    pub fn approve(mut self) -> Result<Post, PendingReviewPost> {
        self.approval_count += 1;

        if self.approval_count == 2 {
            Ok(Post {
                content: self.content,
            })
        } else {
            Err(self)
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
