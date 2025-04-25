use coding_dsa::View;

fn main() {
    println!("Hello, world! - This is ActsOfGL");
    let view = View::new();
    view.render_two_sum();
    view.render_group_anagrams();
    view.render_contains_duplicates();
    view.render_has_nearby_duplicate();
    view.render_has_nearby_divisible_duplicate();
    view.render_has_subarray_with_sum_multiple_of_k();
    view.render_next_larger_element_distances();
    view.render_max_sum_sliding_window();
    view.render_max_sum_with_prefix_sum();
    view.render_get_length_of_shortest_window();
}
