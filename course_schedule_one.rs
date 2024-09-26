use std::collections::HashMap;

type AdjL = HashMap<usize, Vec<usize>>;

fn main() {
    course_schedule_one();
}

fn course_schedule_one() {
    let call = |data: &AdjL| {
        let mut v = vec![0; data.len()];
        let mut p = vec![0; data.len()];
        let r = dfs_cycle(&data, 0, &mut p, &mut v);
        println!("Detect Cycle via DFS: {}", r);
    };

    let data = vec![(0, vec![]), (1, vec![0]), (2, vec![1]), (3, vec![2])]
        .into_iter()
        .collect();
    call(&data);

    let data: AdjL = vec![(0, vec![1]), (1, vec![0])].into_iter().collect();
    call(&data);
}

fn dfs_cycle(d: &AdjL, s: usize, p: &mut Vec<usize>, v: &mut Vec<usize>) -> bool {
    v[s] = 1;
    p[s] = 1;

    for i in d.get(&s).unwrap_or(&vec![]) {
        if v[*i] != 0 {
            if p[*i] == 1 {
                return true;
            }
            continue;
        }

        p[*i] = 1;

        if dfs_cycle(d, *i, p, v) {
            return true;
        }
        p[*i] = 0;
    }

    return false;
}
