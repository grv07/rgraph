use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type AdjL = HashMap<usize, Vec<(usize, usize)>>;

fn main() {
    let data = vec![
        (0, vec![(1, 4), (2, 4)]),
        (1, vec![(0, 4), (2, 2)]),
        (2, vec![(0, 4), (1, 2), (3, 3), (4, 1), (5, 6)]),
        (3, vec![(2, 3), (5, 2)]),
        (4, vec![(2, 1), (5, 5)]),
        (5, vec![(2, 6), (4, 5), (3, 2)]),
    ]
    .into_iter()
    .collect();

    dijkstras_algo(&data, 0);
}

fn dijkstras_algo(data: &AdjL, src: usize) {
    let mut da = vec![usize::MAX; data.len()];
    da[src] = 0;

    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), src));

    while let Some((Reverse(dist), item)) = pq.pop() {
        for (i, d) in data.get(&item).unwrap() {
            let nd = *d + dist;
            if da[*i] < nd {
                continue;
            }

            pq.push((Reverse(nd), *i));
        }

        if da[item] > dist {
            da[item] = dist;
        }
        println!("{pq:?}");
    }
    println!("{da:?}");
}
