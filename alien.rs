use std::collections::{HashMap, VecDeque};

type AdjL = HashMap<usize, Vec<usize>>;

fn main() {
    let d = get_adj(vec!["baa", "abcd", "abca", "cab", "cad"]);
    let mut iov = get_inorder_list(&d);

    let mut q = VecDeque::new();
    for i in 0..iov.len() {
        if iov[i] == 0 {
            q.push_back(i);
        }
    }

    // println!("{d:?}");
    // println!("{iov:?}");
    // println!("{q:?}");

    let rs = kahns_topo_sort(&d, &mut iov, &mut q);
    for r in rs {
        print!("{}", ((r + 97) as u8) as char);
    }
}

fn kahns_topo_sort(d: &AdjL, iov: &mut Vec<usize>, q: &mut VecDeque<usize>) -> Vec<usize> {
    let mut rs = vec![];

    while let Some(item) = q.pop_front() {
        rs.push(item);

        for i in d.get(&item).unwrap() {
            iov[*i] -= 1;

            if iov[*i] == 0 {
                q.push_back(*i);
            }
        }
    }

    rs
}

fn get_adj(l: Vec<&str>) -> AdjL {
    let mut adjl = HashMap::new();

    for i in 1..l.len() {
        let (mut prev, mut next) = (l[i - 1].chars(), l[i].chars());
        while let (Some(p), Some(n)) = (prev.next(), next.next()) {
            if p == n {
                continue;
            } else {
                adjl.entry(p as usize - 97)
                    .and_modify(|v: &mut Vec<usize>| v.push(n as usize - 97))
                    .or_insert(vec![n as usize - 97]);
                // To make sure that we have entry for every vertex
                adjl.entry(n as usize - 97).or_insert(vec![]);

                break;
            }
        }
    }

    adjl
}

fn get_inorder_list(d: &AdjL) -> Vec<usize> {
    let mut iov = vec![0; d.len()];

    for (_k, values) in d.iter() {
        for v in values {
            iov[*v] += 1;
        }
    }

    iov
}
