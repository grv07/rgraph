// https://www.geeksforgeeks.org/problems/shortest-path-in-a-binary-maze-1655453161/1?utm_source=youtube&utm_medium=collab_striver_ytdescription&utm_campaign=shortest-path-in-a-binary-mazew
use std::collections::VecDeque;

fn main() {
    let data = vec![
        vec![1, 1, 1, 1],
        vec![1, 1, 0, 0],
        vec![1, 1, 1, 1],
        vec![0, 0, 0, 1],
        vec![1, 1, 1, 1],
    ];

    let rs = shortestd_in_maze(&data, (0, 1), (4, 0));
    println!("{rs}");

    let data = vec![
        vec![1, 1, 1, 1],
        vec![1, 1, 0, 0],
        vec![1, 1, 1, 1],
        vec![1, 0, 1, 1],
        vec![1, 1, 1, 1],
    ];

    let rs = shortestd_in_maze(&data, (0, 1), (4, 0));
    println!("{rs}");
}

fn shortestd_in_maze(data: &Vec<Vec<usize>>, src: (usize, usize), dest: (usize, usize)) -> i32 {
    let mut q = VecDeque::new();
    q.push_back((src, 0));

    let n = data.len();
    let m = data[0].len();

    let mut da = vec![vec![usize::MAX; m]; n];
    da[src.0][src.1] = 0;

    while let Some(((r, c), d)) = q.pop_front() {
        da[r][c] = d;

        if dest == (r, c) {
            println!("Reach to dest in {d}");
            return d as i32;
        }

        for (dr, dc) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);

            if nr >= 0 && nr < n as i32 && nc >= 0 && nc < m as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if data[nr][nc] == 0 {
                    continue;
                }

                if da[nr][nc] <= d + 1 {
                    continue;
                }

                if da[dest.0][dest.1] < d + 1 {
                    continue;
                }

                q.push_back(((nr, nc), d + 1));
            }
        }
    }

    return -1;
}
