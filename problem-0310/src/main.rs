use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tree = HashMap::<i32, Vec<i32>>::new();

        for edge in edges {
            tree.entry(edge[0]).or_insert(vec![]).push(edge[1]);
            tree.entry(edge[1]).or_insert(vec![]).push(edge[0]);
        }

        while tree.len() > 2 {
            let mut keys: Vec<i32> = vec![];
            for key in tree.keys() {
                if tree.get(key).unwrap().len() == 1 {
                    keys.push(*key);
                }
            }

            let keys = keys;
            for key in keys {
                let to_remove_from = *tree.get(&key).unwrap().get(0).unwrap();
                tree.get_mut(&to_remove_from).unwrap().retain(|&x| x != key);
                tree.remove(&key);
            }
        }

        tree.keys().cloned().collect()
    }
}

fn main() {
    println!("Hello, world!");
}
