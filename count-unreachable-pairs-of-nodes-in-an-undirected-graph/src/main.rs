use std::collections::{HashMap, HashSet, VecDeque};

pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let mut result: i64 = 0;
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut connected_so_far = 0;
    let mut global_visited: HashSet<i32> = HashSet::new();

    for edge in edges {
        let node_1 = edge[0];
        let node_2 = edge[1];

        if !graph.contains_key(&node_1) {
            graph.insert(node_1, Vec::new());
        }

        graph.get_mut(&node_1).unwrap().push(node_2);

        if !graph.contains_key(&node_2) {
            graph.insert(node_2, Vec::new());
        }

        graph.get_mut(&node_2).unwrap().push(node_1);
    }

    for vertex in 0..n {
        if !global_visited.contains(&vertex) {
            let connected = bfs(vertex, &graph, &mut global_visited);
            result = result + connected_so_far * connected as i64;
            connected_so_far = connected_so_far + connected as i64;
        }
    }

    return result;
}

fn bfs(root: i32, tree: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>) -> i32 {
    let mut queue = VecDeque::new();
    visited.insert(root);
    queue.push_back(root);
    let mut connected_nodes = 1;

    while !queue.is_empty() {
        let front = queue.pop_front().unwrap();

        if let Some(neighbors) = tree.get(&front) {
            for node in neighbors {
                if !visited.contains(node) {
                    queue.push_back(*node);
                    connected_nodes = connected_nodes + 1;
                    visited.insert(*node);
                }
            }
        }
    }

    return connected_nodes;
}
fn main() {
    println!("Hello, world!");
}
