use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Node {
    risk: usize,
    idx: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.risk.partial_cmp(&other.risk) {
            Some(Ordering::Equal) => {}
            ord => return ord.map(Ordering::reverse),
        }
        self.idx.partial_cmp(&other.idx).map(Ordering::reverse)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.risk.cmp(&other.risk) {
            Ordering::Equal => {}
            ord => return ord.reverse(),
        }
        self.idx.cmp(&other.idx).reverse()
    }
}

// tag::parse[]
///
/// ```
/// # use mr_kaffee_2021_15::*;
/// assert_eq!((vec![0, 1, 2, 3], 2), parse("01\n23"));
/// ```
pub fn parse(content: &str) -> (Vec<usize>, usize) {
    (
        content
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c as usize - '0' as usize)
            .collect(),
        content.lines().next().expect("No data").len(),
    )
}
// end::parse[]

pub fn solution(grid: &[usize], w: usize, n: usize) -> usize {
    let mut heap = BinaryHeap::new();
    let mut visited = vec![false; grid.len() * n * n];

    heap.push(Node { risk: 0, idx: 0 });
    visited[0] = true;

    while let Some(node) = heap.pop() {
        if node.idx == grid.len() * n * n - 1 {
            return node.risk;
        }

        let (x, y) = (node.idx % (w * n), node.idx / (w * n));

        for (x_a, y_a) in [
            (x + 1, y),
            (x, y + 1),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_sub(1)),
        ] {
            if x_a >= w * n || y_a >= w * n || visited[x_a + y_a * w * n] {
                continue;
            }

            let (x_0, y_0) = (x_a % w, y_a % w);

            visited[x_a + y_a * w * n] = true;
            heap.push(Node {
                risk: node.risk + ((grid[x_0 + y_0 * w] + x_a / w + y_a / w - 1) % 9) + 1,
                idx: x_a + y_a * w * n,
            });
        }
    }

    panic!("No path found");
}

// tag::part1[]
pub fn solution_1(grid: &[usize], width: usize) -> usize {
    solution(grid, width, 1)
}
// end::part1[]

// tag::part2[]
pub fn solution_2(grid: &[usize], width: usize) -> usize {
    solution(grid, width, 5)
}
// end::part2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CONTENT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    const TEST_GRID: &'static [usize] = &[
        1, 1, 6, 3, 7, 5, 1, 7, 4, 2, 1, 3, 8, 1, 3, 7, 3, 6, 7, 2, 2, 1, 3, 6, 5, 1, 1, 3, 2, 8,
        3, 6, 9, 4, 9, 3, 1, 5, 6, 9, 7, 4, 6, 3, 4, 1, 7, 1, 1, 1, 1, 3, 1, 9, 1, 2, 8, 1, 3, 7,
        1, 3, 5, 9, 9, 1, 2, 4, 2, 1, 3, 1, 2, 5, 4, 2, 1, 6, 3, 9, 1, 2, 9, 3, 1, 3, 8, 5, 2, 1,
        2, 3, 1, 1, 9, 4, 4, 5, 8, 1,
    ];
    const TEST_WIDTH: usize = 10;

    #[test]
    fn test_parse() {
        let (grid, width) = parse(TEST_CONTENT);
        assert_eq!(TEST_GRID, grid);
        assert_eq!(TEST_WIDTH, width);
    }

    #[test]
    fn test_solution_1() {
        assert_eq!(40, solution_1(&TEST_GRID, TEST_WIDTH))
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(315, solution_2(&TEST_GRID, TEST_WIDTH))
    }
}
// end::tests[]
