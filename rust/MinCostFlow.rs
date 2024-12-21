#[derive(Clone, Copy)]
struct Edge {
    destination: usize,
    rev_edge_id: usize,
    capacity: u64,
    cost: i64,
}

impl Edge {
    fn new(destination: usize, rev_edge_id: usize, capacity: u64, cost: i64) -> Self {
        return Self {
            destination,
            rev_edge_id,
            capacity,
            cost,
        }
    }
}

struct MinCostGraph {
    n: usize,
    graph: Vec<Vec<Edge>>,
}

impl MinCostGraph {
    fn new(n: usize) -> Self {
        return Self {
            n,
            graph: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dst: usize, capacity: u64, cost: i64) {
        let edge_id = self.graph[src].len();
        let rev_edge_id = self.graph[dst].len();
        self.graph[src].push(Edge::new(dst, rev_edge_id, capacity, cost));
        self.graph[dst].push(Edge::new(src, edge_id, 0u64, -cost));
    }

    fn solve(&mut self, src: usize, dst: usize, mut flow: u64) -> Option<i64> {
        let mut ret = 0i64;
        while 0 < flow {
            let mut dist = vec![None; self.n];
            dist[src] = Some((0i64, None, None));
            let mut in_queue = vec![false; self.n];
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(src);
            in_queue[src] = true;
            while let Some(node) = queue.pop_front() {
                in_queue[node] = false;
                if let Some((cost, _, _)) = dist[node] {
                    for (edge_id, &edge) in self.graph[node].iter().enumerate() {
                        if edge.capacity == 0 {
                            continue;
                        }
                        let cost = cost + edge.cost;
                        if let Some((prev_cost, prev_node, prev_edge_id)) = dist[edge.destination] {
                            if prev_cost <= cost {
                                continue;
                            }
                        }
                        dist[edge.destination] = Some((cost, Some(node), Some(edge_id)));
                        if !in_queue[edge.destination] {
                            in_queue[edge.destination] = true;
                            queue.push_back(edge.destination);
                        }
                    }
                }
            }
            if dist[dst] == None {
                return None;
            }

            let mut routes = vec![];
            let mut current = dst;
            while current != src {
                if let Some((_, node, edge_id)) = dist[current] {
                    assert!(node != None && edge_id != None);
                    let node = node.unwrap();
                    let edge_id = edge_id.unwrap();
                    routes.push((node, edge_id));
                    current = node;
                }
            }

            let f = routes.iter().map(|&(node, edge_id)| self.graph[node][edge_id].capacity).min().unwrap();
            let (unit_cost, _, _) = dist[dst].unwrap();
            flow -= f;
            ret += unit_cost * (f as i64);
            for &(node, edge_id) in &routes {
                self.graph[node][edge_id].capacity -= f;
                let edge = self.graph[node][edge_id];
                self.graph[edge.destination][edge.rev_edge_id].capacity += f;
            }
        }
        return Some(ret);
    }
}

