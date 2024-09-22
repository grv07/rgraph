use std::collections::{HashSet, VecDeque};

type AdjM = Vec<Vec<char>>;

fn main() {
    let ajm = vec![
        vec!['X', 'X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X', 'O'],
        vec!['X', 'X', 'O', 'X', 'O'],
        vec!['X', 'O', 'X', 'O', 'X'],
        vec!['O', 'O', 'X', 'X', 'X'],
    ];

    replace(&ajm, (4, 0));
    replace(&ajm, (1, 4));
    replace(&ajm, (3, 4));
}

fn replace(d: &AdjM, s: (i32, i32)) {
    if d[s.0 as usize][s.1 as usize] == 'X' {
        return;
    }

    let rc = (d.len() - 1) as i32;
    let cc = (d[0].len() - 1) as i32;

    let mut q = VecDeque::new();
    let mut vc = HashSet::new();

    q.push_back(s);
    vc.insert(s);

    while let Some(item) = q.pop_front() {
        let r = item.0;
        let c = item.1;

        let nexts = vec![(r, c), (r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)];
        for n in nexts {
            let r = n.0;
            let c = n.1;

            if r >= 0 && r <= rc && c >= 0 && c <= cc {
                if d[r as usize][c as usize] == 'X' {
                    continue;
                }
                if !vc.insert(n) {
                    continue;
                }

                q.push_back(n);
            }
        }
    }

    println!("{:?}", vc);
}
