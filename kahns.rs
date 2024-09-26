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

    let mut vio = get_vio(&data);
    let mut s = get_stack(&vio);

    let rs = bfs(&data, &mut s, &mut vio);
    println!("Topological order of graph:  {rs:?}");

    let data: AdjL = vec![
        (0, vec![1]),
        (1, vec![2]),
        (2, vec![3, 4]),
        (3, vec![1]),
        (4, vec![]),
    ]
    .into_iter()
    .collect::<HashMap<usize, Vec<usize>>>();

    let mut vio = get_vio(&data);
    let mut s = get_stack(&vio);

    let rs = bfs(&data, &mut s, &mut vio);

    println!("Check if graph have cycle");
    println!("Topological order of graph:  {rs:?}");
}

fn get_stack(vio: &Vec<usize>) -> VecDeque<usize> {
    let mut s = VecDeque::new();

    for i in 0..vio.len() {
        if vio[i] == 0 {
            s.push_back(i);
        }
    }
    s
}

fn get_vio(data: &AdjL) -> Vec<usize> {
    let mut vio = vec![0; data.len()];

    for (_, values) in data.iter() {
        for v in values {
            vio[*v] += 1;
        }
    }

    vio
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
