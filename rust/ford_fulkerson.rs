struct FordFulkerson {
    n: usize,
    graph: Vec<std::collections::HashMap<usize, u64>>,
}

impl FordFulkerson {
    fn new(n: usize) -> Self {
        FordFulkerson {
            n,
            graph: vec![std::collections::HashMap::new(); n],
        }
    }

    fn set_capacity(&mut self, src: usize, dst: usize, capacity: u64) {
        let srcv = capacity + if self.graph[src].contains_key(&dst) {
            self.graph[src][&dst]
        } else {
            0u64
        };
        let dstv = if self.graph[dst].contains_key(&src) {
            self.graph[dst][&src]
        } else {
            0u64
        };
        let diff = std::cmp::min(srcv, dstv);
        self.graph[src].insert(dst, srcv-diff);
        self.graph[dst].insert(src, dstv-diff);
    }

    fn run_flow(&mut self, src: usize, dst: usize, flow: u64) {
        let srcv = self.graph[src][&dst];
        let dstv = self.graph[dst][&src];
        self.graph[src].insert(dst, srcv-flow);
        self.graph[dst].insert(src, dstv-flow);
    }

    fn dfs(&mut self, src: usize, target: usize, flow: u64, mut visited: &mut Vec<bool>) -> u64 {
        if src == target {
            return flow;
        }
        if visited[src] {
            return 0u64;
        }
        visited[src] = true;
        let graph = self.graph[src].clone();
        for (&dst, &cap) in &graph {
            let cap = if cap < flow {
                cap
            } else {
                flow
            };
            if cap == 0 {
                continue;
            }
            let flow = self.dfs(dst, target, cap, &mut visited);
            if flow == 0u64 {
                continue;
            }
            self.run_flow(src, dst, flow);
            return flow;
        }
        return 0u64;
    }

    fn solve(&mut self, src: usize, dst: usize) -> u64 {
        let mut ret = 0u64;
        let max_flow = self.graph[src].values().map(|&x| x).max().unwrap();
        loop {
            let mut visited = vec![false; self.n];
            let flow = self.dfs(src, dst, max_flow, &mut visited);
            if flow == 0u64 {
                break;
            }
            ret += flow;
        }
        return ret;
    }
}
