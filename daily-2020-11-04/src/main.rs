use std::collections::HashMap;

struct Solution {}

impl Solution {
    /// @param n    : the number of nodes in the tree
    /// @param edges: [a_i, b_i] elements, describe an edge a -- b
    /// @return a list of all root nodes that form minimum height trees
    fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }

        let mut total = 0;
        let mut height = f64::log2(n as f64).ceil() as i32;
        let mut heights = vec![height; n as usize];
        let mut neighbors: HashMap<i32, Vec<i32>> = HashMap::new();

        for edge in edges {
            let x = edge[0];
            let y = edge[1];

            neighbors.entry(x).or_insert(vec![]).push(y);
            neighbors.entry(y).or_insert(vec![]).push(x);
        }

        for root in 0..n {
            let mut visited = vec![0; n as usize];
            visited[root as usize] = 1;

            let h = Solution::depth_first_search(root, 1, height, &neighbors, &mut visited);

            heights[root as usize] = h;
            height = if height < h {
                height
            } else if height == h {
                total += 1;
                h
            } else {
                total = 1;
                h
            };

            if total > 2 {
                height = height - 1;
                total = 0;
            }
        }

        let mut answer = vec![];
        for root in 0..n {
            if heights[root as usize] == height {
                answer.push(root);
            }
        }

        answer
    }

    fn depth_first_search(
        node: i32,
        cur_height: i32,
        cur_min_height: i32,
        neighbors: &HashMap<i32, Vec<i32>>,
        visited: &mut Vec<i32>,
    ) -> i32 {
        if cur_height > cur_min_height {
            return cur_min_height + 1;
        }

        let mut height = cur_height;

        for neighbor in &neighbors[&node] {
            let i = *neighbor as usize;
            if visited[i] != 0 {
                continue;
            }

            visited[i] = 1;

            let h = Solution::depth_first_search(
                *neighbor,
                cur_height + 1,
                cur_min_height,
                neighbors,
                visited,
            );

            height = if height > h { height } else { h };
        }

        height
    }
}

fn main() {
    let n = 6;
    let edges: Vec<Vec<i32>> = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];

    // let n = 2;
    // let edges: Vec<Vec<i32>> = vec![vec![0, 1]];

    println!("{:#?}", Solution::find_min_height_trees(n, edges));

    // println!("{}", f64::log2(3.0));
}
