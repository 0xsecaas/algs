mod algorithms;

fn main() {
    use algorithms::strings::merge_alternately;

    println!(
        "Leetcode XOR result: {}",
        merge_alternately("abcd".to_string(), "abcd".into())
    );
}
