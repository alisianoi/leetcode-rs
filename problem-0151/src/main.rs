struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut xs = s
            .split(" ")
            .map(|word| word.trim())
            .filter(|word| !word.is_empty())
            .collect::<Vec<_>>();
        xs.reverse();
        xs.join(" ")
    }
}

fn main() {
    println!(
        "{}",
        Solution::reverse_words(String::from("    hello    world"))
    );
}
