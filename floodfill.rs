use std::collections::{HashSet, VecDeque};

fn bfs(g: &mut Vec<Vec<i32>>, i: (i32, i32), nc: i32) {
    let ic = g[i.0 as usize][i.1 as usize];

    let mut stack = VecDeque::from(vec![i]);
    let mut vc = HashSet::new();

    while let Some(item) = stack.pop_front() {
        let r = item.0;
        let c = item.1;

        let points = |r, c| -> Vec<(i32, i32)> {
            let mut v = vec![];
            for r in r - 1..r + 2 {
                v.push((r, c));
            }
            for c in c - 1..c + 2 {
                v.push((r, c));
            }

            v
        };

        for p in points(r, c) {
            let r = p.0;
            let c = p.1;

            if r >= 0 && r <= 2 && c >= 0 && c <= 2 {
                if vc.insert((r, c)) {
                    if g[r as usize][c as usize] == ic {
                        g[r as usize][c as usize] = nc;
                        stack.push_back((r, c));
                    }
                }
            }
        }
    }
}

fn main() {
    let mut g = vec![vec![1, 1, 0], vec![2, 2, 0], vec![2, 2, 2]];
    bfs(&mut g, (2, 0), 3);
    println!("{:?}", g);

    let mut g = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
    bfs(&mut g, (1, 1), 2);
    println!("{:?}", g);
}
