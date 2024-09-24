use std::collections::HashMap;

type AjL = HashMap<usize, Vec<usize>>;

fn main() {
    let mut data: AjL = vec![(0, vec![1, 2]), (1, vec![2]), (2, vec![3]), (3, vec![])]
        .into_iter()
        .collect::<HashMap<usize, Vec<usize>>>();

    let mut v = vec![0; data.len()];
    let mut p = vec![0; data.len()];

    let out = dfs(&mut data, 0, &mut v, &mut p);
    println!("Output: {:?}", out);

    let mut data: AjL = vec![(0, vec![1]), (1, vec![2]), (2, vec![3]), (3, vec![3])]
        .into_iter()
        .collect::<HashMap<usize, Vec<usize>>>();

    let mut v = vec![0; data.len()];
    let mut p = vec![0; data.len()];

    let out = dfs(&mut data, 0, &mut v, &mut p);
    println!("Output: {:?}", out);
}

fn dfs(d: &AjL, r: usize, v: &mut Vec<usize>, p: &mut Vec<usize>) -> bool {
    p[r] = 1;
    v[r] = 1;

    for ni in d.get(&r).unwrap() {
        if v[*ni] != 0 {
            if p[*ni] == 1 {
                return true;
            }
            continue;
        }

        if dfs(d, *ni, v, p) {
            return true;
        }

        p[*ni] = 0;
    }

    return false;
}
