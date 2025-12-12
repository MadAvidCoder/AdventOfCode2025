use kiddo::{KdTree, SquaredEuclidean};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct DSU {
    rep: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            rep: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, k: usize) -> usize {
        if self.rep[k] != k {
            self.rep[k] = self.find(self.rep[k]);
        }
        self.rep[k]
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut x_root = self.find(x);
        let mut y_root = self.find(y);

        if x_root == y_root { return; }

        if self.size[x_root] < self.size[y_root] {
            std::mem::swap(&mut x_root, &mut y_root);
        }

        self.rep[y_root] = x_root;
        self.size[x_root] += self.size[y_root];
        self.components -= 1;
    }

    fn get_sizes(&mut self) -> Vec<usize> {
        let mut sizes = vec![0; self.rep.len()];
        for i in 0..self.rep.len() {
            sizes[self.find(i)] += 1;
        }
        sizes.into_iter().filter(|&c| c > 0).collect()
    }
}

pub fn part1(input: &str) -> u32 {
    let points: Vec<[f64;3]> = input
        .lines()
        .map(|line| {
            let coords = line
                .split(',')
                .map(|num| num
                    .trim()
                    .parse::<f64>()
                    .unwrap()
                )
                .collect::<Vec<f64>>();
            [coords[0], coords[1], coords[2]]
        })
        .collect();


    let mut tree: KdTree<f64, 3> = KdTree::new();

    for (i, point) in points.iter().enumerate() {
        tree.add(point, i as u64);
    }

    let mut edges = Vec::new();

    for (i, point) in points.iter().enumerate() {
        let neighbours = tree.nearest_n::<SquaredEuclidean>(point, points.len());

        for n in neighbours {
            if n.item as usize > i {
                edges.push((n.distance as i64, i, n.item as usize));
            }
        }
    }

    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let shortest = &edges[..edges.len().min(1000)];

    let mut dsu = DSU::new(points.len());

    for &(_, a, b) in shortest {
        dsu.union(a, b);
    }

    let mut sizes = dsu.get_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    sizes.iter().take(3).product::<usize>() as u32
}

pub fn part2(input: &str) -> u64 {
    let points: Vec<[f64;3]> = input
        .lines()
        .map(|line| {
            let coords = line
                .split(',')
                .map(|num| num
                    .trim()
                    .parse::<f64>()
                    .unwrap()
                )
                .collect::<Vec<f64>>();
            [coords[0], coords[1], coords[2]]
        })
        .collect();


    let mut tree: KdTree<f64, 3> = KdTree::new();

    for (i, point) in points.iter().enumerate() {
        tree.add(point, i as u64);
    }

    let mut neighbours: Vec<Vec<(i64, usize)>> = Vec::with_capacity(points.len());
    let mut cursor: Vec<usize> = vec![1usize; points.len()];
    let mut heap: BinaryHeap<Reverse<(i64, usize, usize)>> = BinaryHeap::new();

    for point in points.iter() {
        neighbours.push(tree
            .nearest_n::<SquaredEuclidean>(point, points.len())
            .into_iter()
            .map(|n| (n.distance as i64, n.item as usize))
            .collect::<Vec<(i64, usize)>>()
        );
    }

    for i in 0..points.len() {
        let (dist, b) = neighbours[i][1];
        heap.push(Reverse((dist, i, b)));
    }

    let mut dsu: DSU = DSU::new(points.len());
    let mut last_edge: (usize, usize) = (0, 0);

    while dsu.components > 1 {
        let Reverse((dist, a, b)) = heap.pop().unwrap();

        if dsu.find(a) != dsu.find(b) {
            dsu.union(a, b);
            last_edge = (a, b);
        }

        cursor[a] += 1;

        if cursor[a] < neighbours[a].len() {
            let (dist, b) = neighbours[a][cursor[a]];
            heap.push(Reverse((dist, a, b)));
        }
    }

    points[last_edge.0][0] as u64 * points[last_edge.1][0] as u64
}