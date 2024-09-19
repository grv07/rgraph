use std::collections::{HashMap, VecDeque};

type Nodes = HashMap<usize, Vec<usize>>;

struct Graph {
    root: usize,
    nodes: Nodes,
}

impl Graph {
    fn new(root: usize, nodes: Nodes) -> Self {
        Self { root, nodes }
    }

    fn dfs(&self, v: &usize, cache: &mut [usize]) {
        if let Some(childs) = self.nodes.get(&v) {
            println!("{:?}", v);
            for child in childs {
                if cache[child - 1] == 1 {
                    continue;
                }
                cache[child - 1] = 1;
                self.dfs(child, cache);
            }
        } else {
            println!("{:?}", v);
        }
    }

    fn bfs(&self) {
        let mut t_cache = vec![0; self.nodes.len()];
        let mut q = VecDeque::new();
        q.push_back(self.root);

        while !q.is_empty() {
            if let Some(item) = q.pop_front() {
                if t_cache[item - 1] == 1 {
                    continue;
                }

                println!("{item}");
                t_cache[item - 1] = 1;

                if let Some(childs) = self.nodes.get(&item) {
                    for child in childs {
                        q.push_back(*child);
                    }
                }
            }
        }
    }
}

fn main() {
    let nodes: Nodes = vec![
        (1, vec![2, 6]),
        (2, vec![1, 3, 4]),
        (6, vec![1, 7, 9]),
        (3, vec![2]),
        (4, vec![2, 5]),
        (5, vec![8, 4]),
        (7, vec![6, 8]),
        (9, vec![6]),
        (8, vec![5, 7]),
    ]
    .into_iter()
    .collect();

    println!("{:?}", nodes);

    let g = Graph::new(1, nodes);
    g.bfs();

    let nodes_dfs: Nodes = vec![
        (1, vec![2, 3]),
        (2, vec![1, 5, 6]),
        (3, vec![1, 7, 4]),
        (4, vec![3, 8]),
        (5, vec![2]),
        (6, vec![2]),
        (7, vec![3, 8]),
        (8, vec![4, 7]),
    ]
    .into_iter()
    .collect();

    println!("{:?}", nodes_dfs);

    let g = Graph::new(1, nodes_dfs);
    // g.bfs();
    let mut t_cache = vec![0; g.nodes.len()];
    t_cache[g.root - 1] = 1;
    g.dfs(&g.root, &mut t_cache);
}
