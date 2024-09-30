use std::collections::{HashMap, VecDeque};

type AdjL = HashMap<usize, Vec<usize>>;

fn main() {
    let data: AdjL = vec![
        (0, vec![1, 3]),
        (1, vec![0, 2, 3]),
        (2, vec![1, 6]),
        (3, vec![0, 4]),
        (4, vec![3, 5]),
        (5, vec![4, 6]),
        (6, vec![2, 5, 7, 8]),
        (7, vec![6, 8]),
        (8, vec![6, 7]),
    ]
    .into_iter()
    .collect();

    bfs(&data, (0, 0));
    // println!("{data:?}");
}

fn bfs(data: &AdjL, s: (usize, usize)) {
    let mut q = VecDeque::new();
    q.push_back(s);

    let mut d = vec![usize::MAX; data.len()];
    d[s.0] = s.1;

    while let Some(item) = q.pop_front() {
        let (item, dist) = item;
        for i in data.get(&item).unwrap() {
            if d[*i] < dist + 1 {
                continue;
            }

            d[*i] = dist + 1;
            q.push_back((*i, dist + 1));
            // print!("{:?} ", i);
        }
    }

    println!("{:?}", d);
}
