use hello_rust::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");

    let pending_post = post.request_review();

    let post = pending_post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}