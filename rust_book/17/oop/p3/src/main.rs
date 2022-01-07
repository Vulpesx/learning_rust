use p3::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today!");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    // post.reject();
    assert_eq!("I ate a salad for lunch today!", post.content());
}
