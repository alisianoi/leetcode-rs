struct Solution {}

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let s = s.into_bytes();

        let mut best = 0;
        let mut curr = 1;
        let mut prev = s[0];
        for i in 1..s.len() {
            if s[i] == prev {
                curr += 1;
            } else {
                best = if best >= curr { best } else { curr };
                curr = 1;
                prev = s[i];
            }
        }

        if best >= curr {
            best
        } else {
            curr
        }
    }
}

fn main() {
    println!("{}", Solution::max_power(String::from("123aaabbbbee")));
}
