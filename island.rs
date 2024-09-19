use std::collections::{HashSet, VecDeque};

fn graph() -> Vec<Vec<i32>> {
    vec![
        vec![0, 1, 1, 0],
        vec![0, 1, 1, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 0],
        vec![1, 1, 0, 1],
    ]
}

fn main() {
    let g = graph();
    count_island(g);
}

fn dfs(data: &Vec<Vec<i32>>, root: (i32, i32)) -> bool {
    println!("root: {root:?}");

    let mut tc = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(root);

    while let Some(item) = queue.pop_back() {
        let r = item.0;
        let c = item.1;

        for r in r - 1..r + 2 {
            for c in c - 1..c + 2 {
                if r >= 0 && c >= 0 && r <= 4 && c <= 3 {
                    if data[r as usize][c as usize] == 1 {
                        // if already in cache
                        if !tc.insert((r, c)) {
                            continue;
                        }
                        queue.push_front((r, c));

                        print!("{r}{c} ");
                    }
                }
            }
        }
    }
    println!("");

    return true;
}

fn count_island(data: Vec<Vec<i32>>) {
    for point in vec![(0, 1), (4, 0), (4, 3)] {
        dfs(&data, point);
    }
}
