struct Solution {}

impl Solution {
    // 1 <= position.length <= 100
    // 1 <= position[i] <= 10^9
    pub fn min_cost_to_move_chips(positions: Vec<i32>) -> i32 {
        let mut one_count = 0;
        let mut zero_count = 0;

        for i in 0..positions.len() {
            if positions[i] % 2 == 0 {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }

        if one_count > zero_count {
            zero_count
        } else {
            one_count
        }
    }
}

fn main() {
    println!("Hello, world!");
}
