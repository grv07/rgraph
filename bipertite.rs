use std::collections::{HashMap, VecDeque};

type AdjL = HashMap<usize, Vec<usize>>;

fn main() {
    let data: AdjL = vec![
        (0, vec![1, 2, 3]),
        (1, vec![0, 2]),
        (2, vec![0, 3, 1]),
        (3, vec![0, 2]),
    ]
    .into_iter()
    .collect::<HashMap<usize, Vec<usize>>>();

    println!("Is bipertite via BFS: {}", bfs(&data, 0));
    println!("Is bipertite via DFS: {}", dfs(&data, 0));

    println!("");

    let data: AdjL = vec![
        (0, vec![1, 3]),
        (1, vec![0, 2]),
        (2, vec![3, 1]),
        (3, vec![0, 2]),
    ]
    .into_iter()
    .collect::<HashMap<usize, Vec<usize>>>();

    println!("Is bipertite via BFS: {}", bfs(&data, 0));
    println!("Is bipertite via DFS: {}", dfs(&data, 0));
}

fn bfs(d: &AdjL, s: usize) -> bool {
    let mut c = 1;
    let mut q = VecDeque::new();
    let mut v = vec![-1; d.len()];
    q.push_back(s);

    v[s] = c % 2;

    while let Some(item) = q.pop_front() {
        c += 1;
        let color = c % 2;

        for i in d.get(&item).unwrap() {
            if v[*i] != -1 {
                if v[item] == color {
                    return false;
                }
                continue;
            }

            v[item] = color;

            q.push_back(*i);
        }
    }

    return true;
}

fn dfs(d: &AdjL, root: usize) -> bool {
    let mut c = 1;
    let mut s = VecDeque::new();
    s.push_back(root);

    let mut v = vec![-1; d.len()];

    while let Some(item) = s.pop_back() {
        let color = c % 2;
        for al in d.get(&item).unwrap() {
            if v[*al] != -1 {
                if v[item] == color {
                    return false;
                }
                continue;
            }
            v[item] = color;
            s.push_back(*al);
        }
        c += 1;
    }
    return true;
}
