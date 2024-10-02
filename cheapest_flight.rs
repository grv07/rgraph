use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type AdjL = HashMap<usize, Vec<(usize, usize)>>;

fn main() {
    let data = vec![
        (0, vec![(1, 5), (3, 2)]),
        // (1, vec![(2, 5), (4, 1)]),
        (1, vec![(4, 1)]),
        (2, vec![]),
        (3, vec![(1, 2)]),
        (4, vec![(5, 1)]),
        (4, vec![(2, 1)]),
    ]
    .into_iter()
    .collect();

    let opt = shortest_in_k_stops(&data, 0, 2, 2);
    println!("{opt}");
}

fn shortest_in_k_stops(data: &AdjL, src: usize, dest: usize, k: usize) -> i32 {
    let stops = 0;
    let mut da = vec![usize::MAX; data.len()];
    da[src] = 0;

    let mut pq = BinaryHeap::new();
    pq.push((Reverse(stops), (src, 0)));

    while let Some((Reverse(stops), (item, d))) = pq.pop() {
        for (i, dist) in data.get(&item).unwrap() {
            if k <= stops && !i != dest {
                continue;
            }

            pq.push((Reverse(stops + 1), (*i, *dist + d)));
        }

        if da[item] > d {
            da[item] = d;
        }
    }

    if da[dest] == usize::MAX {
        return -1;
    }

    da[dest] as i32
}
