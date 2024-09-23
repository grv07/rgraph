use std::collections::{HashMap, VecDeque};

type AdjL = HashMap<i32, Vec<i32>>;

fn main() {
    let data: AdjL = vec![
        (0, vec![1, 2, 3]),
        (1, vec![0, 2]),
        (2, vec![0, 3, 1]),
        (3, vec![0, 2]),
    ]
    .into_iter()
    .collect::<HashMap<i32, Vec<i32>>>();

    println!("Is bipertite : {}", bfs(&data, 0));

    let data: AdjL = vec![
        (0, vec![1, 3]),
        (1, vec![0, 2]),
        (2, vec![3, 1]),
        (3, vec![0, 2]),
    ]
    .into_iter()
    .collect::<HashMap<i32, Vec<i32>>>();

    println!("Is bipertite : {}", bfs(&data, 0));
}

fn bfs(d: &AdjL, s: i32) -> bool {
    let mut c = 1;
    let mut q = VecDeque::new();
    let mut v = vec![-1; d.len()];
    q.push_back(s);

    v[s as usize] = c % 2;

    while let Some(item) = q.pop_front() {
        c += 1;
        let color = c % 2;

        for i in d.get(&item).unwrap() {
            if v[*i as usize] != -1 {
                if v[item as usize] == color {
                    return false;
                }
                continue;
            }

            v[item as usize] = color;

            q.push_back(*i);
        }
    }

    return true;
}

// fn dfs() {}
