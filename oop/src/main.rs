use oop::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    // reject on first draft
    let post = post.request_review().reject();

    // approve until it prints
    let post = post.request_review();

    let mut post = post.approve();
    let post = loop {
        match post {
            Ok(post) => break post,
            Err(draft) => {
                println!("insufficient approvals...");
                post = draft.approve();
            }
        }
    };

    println!("{}", post.content());
}
