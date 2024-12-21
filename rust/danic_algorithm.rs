use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct Edge {
    destination: usize,
    rev_edge_id: usize,
    capacity: u64,
}

impl Edge {
    fn new(destination: usize, rev_edge_id: usize, capacity: u64) -> Self {
        return Self {
            destination,
            rev_edge_id,
            capacity,
        }
    }
}

struct DinicAlgorithm {
    n: usize,
    graph: Vec<Vec<Edge>>,
}

impl DinicAlgorithm {
    fn new(n: usize) -> Self {
        return Self {
            n,
            graph: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dst: usize, capacity: u64) {
        let edge_id = self.graph[src].len();
        let rev_edge_id = self.graph[dst].len();
        self.graph[src].push(Edge::new(dst, rev_edge_id, capacity));
        self.graph[dst].push(Edge::new(src, edge_id, 0u64));
    }

    fn bfs(&self, src: usize) -> Vec<Option<u64>> {
        let mut levels = vec![None; self.n];
        let mut queue = VecDeque::new();
        queue.push_back((src, 0u64));
        while let Some((node, d)) = queue.pop_front() {
            if levels[node].is_some() {
                continue;
            }
            levels[node] = Some(d);
            for &edge in &self.graph[node] {
                if 0 < edge.capacity {
                    queue.push_back((edge.destination, d+1));
                }
            }
        }
        return levels;
    }

    fn dfs(&mut self, src: usize, target: usize, flow: Option<u64>, levels: &Vec<Option<u64>>) -> u64 {
        if src == target {
            return flow.or(Some(0u64)).unwrap();
        }
        for edge_id in 0..self.graph[src].len() {
            let edge = self.graph[src][edge_id];
            if edge.capacity == 0 || levels[src] == None || levels[edge.destination] == None
                || levels[edge.destination].unwrap() <= levels[src].unwrap() {
                    continue;
                }
            let flow = self.dfs(
                edge.destination,
                target,
                flow.or(Some(edge.capacity)).map(|v| v.min(edge.capacity)),
                &levels);
            self.graph[src][edge_id].capacity -= flow;
            self.graph[edge.destination][edge.rev_edge_id].capacity += flow;
            if 0 < flow {
                return flow;
            }
        }
        return 0u64;
    }

    fn solve(&mut self, src: usize, dst: usize) -> u64 {
        let mut flow = 0u64;
        loop {
            let levels = self.bfs(src);
            if levels[dst] == None {
                return flow;
            }
            loop {
                let f = self.dfs(src, dst, None, &levels);
                if 0 < f {
                    flow += f;
                } else {
                    break;
                }
            }
        }
    }
}

