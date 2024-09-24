use std::collections::HashMap;

type AdjL = HashMap<usize, Vec<usize>>;

fn main() {
    let data = vec![
        (0, vec![]),
        (1, vec![]),
        (2, vec![3]),
        (3, vec![1]),
        (5, vec![0, 2]),
        (4, vec![0, 1]),
    ]
    .into_iter()
    .collect::<HashMap<usize, Vec<usize>>>();

    let mut rs = vec![];
    let mut v = vec![0; data.len()];
    for k in 0..data.len() {
        if v[k] != 0 {
            continue;
        }
        dfs(&data, k, &mut rs, &mut v);
        println!("{:?}", rs);
    }
}

fn dfs(d: &AdjL, s: usize, rs: &mut Vec<usize>, v: &mut Vec<usize>) {
    v[s] = 1;

    for i in d.get(&s).unwrap() {
        if v[*i] != 0 {
            continue;
        }
        dfs(d, *i, rs, v);
    }

    rs.push(s);
}
