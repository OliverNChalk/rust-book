use oop::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    // reject on first draft
    let post = post.request_review().reject();

    // approve until it prints
    let mut post = post.request_review();
    loop {
        match post.approve() {
            Ok(approved) => {
                println!("{}", approved.content());
                break;
            }
            Err(insufficient) => {
                println!("insufficient approvals, continuing");
                post = insufficient;
                continue;
            }
        }
    }
}
