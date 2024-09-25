use std::collections::{HashMap, VecDeque};

type AdjL = HashMap<usize, Vec<usize>>;

fn main() {
    let data: AdjL = vec![
        (0, vec![]),
        (1, vec![]),
        (2, vec![3]),
        (3, vec![1]),
        (4, vec![0, 1]),
        (5, vec![0, 2]),
    ]
    .into_iter()
    .collect::<HashMap<usize, Vec<usize>>>();

    let mut vio = vec![0; data.len()];
    let mut s = VecDeque::new();

    for (_, values) in data.iter() {
        for v in values {
            vio[*v] += 1;
        }
    }

    for i in 0..vio.len() {
        if vio[i] == 0 {
            // println!("{i}");
            s.push_back(i);
        }
    }

    let rs = bfs(&data, &mut s, &mut vio);
    println!("Topological order of graph:  {rs:?}");
}

fn bfs(d: &AdjL, s: &mut VecDeque<usize>, vio: &mut Vec<usize>) -> Vec<usize> {
    let mut rs = vec![];
    while let Some(item) = s.pop_front() {
        rs.push(item);
        for i in d.get(&item).unwrap() {
            vio[*i] -= 1;

            if vio[*i] == 0 {
                s.push_back(*i);
            }
        }
    }

    rs
}
