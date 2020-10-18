use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let numerator = numerator as i64;
        let denominator = denominator as i64;

        let x = numerator / denominator;
        let y = numerator % denominator;

        if y == 0 {
            return format!("{}", x);
        }

        // y != 0 => there was at least one division
        // y != 0 => denominator != 1
        let negative = (numerator < 0) ^ (denominator < 0);
        let x = (x as i64).abs();
        let mut y = (y as i64).abs();
        let denominator = (denominator as i64).abs();

        let mut step: usize = 0;
        let mut seen: HashMap<i64, usize> = HashMap::new();
        let mut fraction = String::new();

        loop {
            if let Some(i) = seen.get(&y) {
                let i = *i;
                let prefix = &fraction[0..i];
                let suffix = &fraction[i..];
                return if negative {
                    format!("-{}.{}({})", x, prefix, suffix)
                } else {
                    format!("{}.{}({})", x, prefix, suffix)
                };
            }

            seen.insert(y, step);

            let v = 10 * y / denominator;
            y = 10 * y - v * denominator;

            fraction.push_str(&v.to_string());

            if y == 0 {
                break;
            } else {
                step += 1;
            }
        }

        if negative {
            format!("-{}.{}", x, fraction)
        } else {
            format!("{}.{}", x, fraction)
        }
    }
}

// tests
//
// i32::MIN, 1 = i32::MIN
// i32::MIN, i32::MIN = 1
// i32::MIN / 2, i32::MIN = 0.5
// i32::MIN + 1, i32::MIN = something close to zero, positive
fn main() {
    // println!("{}", Solution::fraction_to_decimal(i32::MIN, 1));
    // println!("{}", Solution::fraction_to_decimal(i32::MIN, -1));
    // println!("{}", Solution::fraction_to_decimal(i32::MIN, i32::MIN));
    // println!("{}", Solution::fraction_to_decimal(i32::MIN / 2, i32::MIN));
    // println!("{}", Solution::fraction_to_decimal(i32::MIN + 1, i32::MIN));
    // println!("{}", Solution::fraction_to_decimal(-1, -3));
    // println!("{}", Solution::fraction_to_decimal(-113, 117));
    // println!("{}", Solution::fraction_to_decimal(1, -12));
    let x = i32::MIN;
    let y = -1;
    println!("{}", x / y);
    // let s = String::from("hello");
    // println!("{}", &s[0..2]);

    // let mut m: HashMap<i64, i32> = HashMap::new();

    // m.insert(0, 2);

    // let s = String::from("hello");

    // if let Some(i) = m.get(&0) {
    //     let i = *i as usize;
    //     println!("good: {}", &s[0..i]);
    // } else {
    //     println!("bad");
    // }
}
