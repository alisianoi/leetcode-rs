use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let version1: Vec<i32> = version1
            .split('.')
            .map(|revision| revision.parse::<i32>().unwrap())
            .collect();
        let version2: Vec<i32> = version2
            .split('.')
            .map(|revision| revision.parse::<i32>().unwrap())
            .collect();

        let limit = min(version1.len(), version2.len());

        for i in 0..limit {
            if version1[i] < version2[i] {
                return -1;
            }
            if version1[i] > version2[i] {
                return 1;
            }
        }

        for i in limit..version1.len() {
            if version1[i] > 0 {
                return 1;
            }
        }

        for i in limit..version2.len() {
            if version2[i] > 0 {
                return -1;
            }
        }

        0
    }
}

fn main() {
    let solution = Solution::compare_version("1.1.1".to_string(), "1.1".to_string());

    println!("{}", solution);

    for i in 3..0 {
        println!("{}", i);
    }
}
