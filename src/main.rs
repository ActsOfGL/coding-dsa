mod view;
mod helpers;
use view::View;

fn main() {
    println!("Hello, world! - This is ActsOfGL");
    let view = View::new();
    view.render_two_sum();
    view.render_group_anagrams();
    view.render_contains_duplicates();
}
