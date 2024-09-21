use std::collections::{HashSet, VecDeque};

type Ajm = Vec<Vec<i32>>;

fn main() {
    let data = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 0, 1]];

    let level = dnc(&data, (1, 1));
    println!("Data level: {level}");
}

fn dnc(data: &Ajm, s: (i32, i32)) -> i32 {
    let mut vc = HashSet::new();
    let mut q: VecDeque<((i32, i32), i32)> = VecDeque::new();

    q.push_back((s, 0));

    while let Some(((r, c), level)) = q.pop_front() {
        if data[r as usize][c as usize] == 1 {
            // println!(">> {level}");
            return level;
        }

        let level = level + 1;
        let rn = data.len() as i32;
        let cn = data[0].len() as i32;

        let rs = vec![(r, c), (r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)];

        for (r, c) in rs {
            if r >= 0 && r < rn && c >= 0 && c < cn {
                if !vc.insert((r, c)) {
                    continue;
                }
                // println!("{} {} > {} {level}", r, c, data[r as usize][c as usize]);
                q.push_back(((r, c), level));
            }
        }
    }

    -1
}
