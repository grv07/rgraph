use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type AdjL = HashMap<usize, Vec<(usize, usize)>>;

fn main() {
    let data = vec![
        (0, vec![(1, 1), (2, 2), (3, 1), (4, 2)]),
        (1, vec![(5, 2)]),
        (2, vec![(5, 1)]),
        (3, vec![(5, 2), (7, 3), (6, 2)]),
        (4, vec![(6, 1)]),
        (5, vec![(8, 1)]),
        (6, vec![(8, 1)]),
        (7, vec![(8, 1)]),
        (8, vec![]),
    ]
    .into_iter()
    .collect();

    let rs = shotest_paths(&data, 0, 8);
    println!("Shortest Path count: {rs}");
}

fn shotest_paths(data: &AdjL, src: usize, dest: usize) -> i32 {
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), src));

    let mut da = vec![usize::MAX; data.len()];
    da[src] = 0;

    let mut count = 1;

    while let Some((Reverse(d), item)) = pq.pop() {
        // println!("{item}");
        for (i, dist) in data.get(&item).unwrap() {
            if da[*i] < d + dist {
                continue;
            }

            pq.push((Reverse(d + dist), *i));
        }

        if da[item] >= d {
            if item == dest {
                if da[dest] == d {
                    count += 1;
                } else {
                    count = 1;
                }
            }
            da[item] = d;
        }

        // println!("C: {count:?}");
    }

    count
}
