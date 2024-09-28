use std::collections::HashMap;

type AdjL = HashMap<usize, Vec<(usize, usize)>>;

fn main() {
    let src = 6;
    let data: AdjL = vec![
        (0, vec![(1, 2)]),
        (1, vec![(3, 1)]),
        (2, vec![(3, 3)]),
        (3, vec![]),
        (4, vec![(0, 3), (2, 1)]),
        (5, vec![(4, 1)]),
        (6, vec![(4, 2), (5, 3)]),
    ]
    .into_iter()
    .collect();

    let mut v = vec![0; data.len()];

    let mut rs = vec![];

    dfs_to(&data, src, &mut v, &mut rs);
    println!("{rs:?}");

    let mut d = vec![usize::MAX; data.len()];
    d[src] = 0;

    while let Some(item) = rs.pop() {
        for (i, dl) in data.get(&item).unwrap() {
            let nd = d[item] + dl;
            if nd < d[*i] {
                d[*i] = nd;
            }
        }
    }

    println!("{d:?}");
}

fn dfs_to(data: &AdjL, s: usize, v: &mut Vec<usize>, rs: &mut Vec<usize>) {
    v[s] = 1;

    for (i, _d) in data.get(&s).unwrap() {
        if v[*i] != 0 {
            continue;
        }

        dfs_to(data, *i, v, rs);
    }

    rs.push(s);
}
