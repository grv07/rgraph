use std::collections::{HashMap, VecDeque};

type Adj = HashMap<i32, Vec<i32>>;

fn main() {
    let data: Adj = vec![
        (1, vec![2, 3]),
        (2, vec![5, 1]),
        (3, vec![1, 4, 6]),
        (4, vec![3]),
        (5, vec![2, 7]),
        (6, vec![3, 7]),
        (7, vec![5, 6]),
    ]
    .into_iter()
    .collect::<Adj>();

    println!("Check Cycle:");
    println!("{data:#?}");

    let p = is_cycle(data, 1);
    println!(">> {p}");
}

fn is_cycle(data: Adj, r: i32) -> bool {
    let n = data.len() + 1;

    let mut q = VecDeque::new();
    q.push_back(r);

    let mut vc = vec![0; n];

    while let Some(item) = q.pop_front() {
        let mut c_c = 0;

        for v in data.get(&item).unwrap() {
            if vc[*v as usize] == 1 {
                c_c += 1;
                continue;
            } else {
                q.push_back(*v);
            }
        }

        if c_c == 2 {
            return true;
        }

        vc[item as usize] = 1;
    }

    return false;
}
