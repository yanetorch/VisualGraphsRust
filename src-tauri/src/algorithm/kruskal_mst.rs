//! Построение минимального остовного дерева
//! при помощи DSU

use crate::model::graph::Graph;

struct DSU {
    // размер
    siz: Vec<usize>,
    // адрес
    f: Vec<usize>,
}

impl DSU {
    fn new(size: usize) -> Self {
        let siz = vec![ 1; size ];
        
        let mut f = Vec::new();

        {
            let mut p = 0;
            f.resize_with(size, || { p += 1; p - 1 });
        }

        DSU {
            siz,
            f,
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while x != self.f[x] {
            self.f[x] = self.f[self.f[x]];
            x = self.f[x];
        }
        x
    }

    fn merge(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.find(x);
        y = self.find(y);
        if x == y {
            return false;
        }
        if self.siz[y] > self.siz[x] {
            (x, y) = (y, x);
        }

        self.siz[x] += self.siz[y];
        self.f[y] = x;

        true
    }
}

// Ребро между вершиной A и B с весом weight
#[derive(Debug)]
struct Edge {
    a: usize,
    b: usize,
    weight: usize,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.weight == other.weight
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

fn kruskal_find_mst(mut edges: Vec<Edge>, size: usize) -> Graph {
    edges.sort_by(| a, b | {
        if a == b {
            return std::cmp::Ordering::Equal;
        }
        if a > b {
            return std::cmp::Ordering::Greater;
        }
        std::cmp::Ordering::Less
    });
    let mut dsu = DSU::new(size);
    let mut graph = Graph::new(size);
    for edge in edges {
        if dsu.merge(edge.a, edge.b) {
            graph.add_edge(edge.a, edge.b).ok();
            graph.add_edge(edge.b, edge.a).ok();
        }
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_how_iota_init_vector() {
        let mut f = Vec::new();
        let mut p = 0usize;
        f.resize_with(15, || { p += 1; p - 1 });
        let mut ok = true;
        for (i, value) in f.into_iter().enumerate() {
            ok &= value == i;
        }
        assert!(ok);        
    }
}