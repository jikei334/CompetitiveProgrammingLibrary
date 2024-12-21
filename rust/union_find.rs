struct UnionFind {
    sizes: Vec<usize>,
    roots: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            sizes: vec![1usize; n],
            roots: (0..n).collect::<Vec<_>>(),
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.roots[x] != x {
            self.roots[x] = self.root(self.roots[x] as usize);
        }
        return self.roots[x];
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        return self.root(x) == self.root(y)
    }

    fn size(&mut self, x: usize) -> usize {
        let root_x = self.root(x);
        return self.sizes[root_x];
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        if self.is_same(x, y) {
            return false;
        } else {
            let mut root_x = self.root(x);
            let mut root_y = self.root(y);
            let (root_x, root_y) = if self.size(root_x) < self.size(root_y) {
                (root_y, root_x)
            } else {
                (root_x, root_y)
            };
            self.sizes[root_x] += self.size(root_y);
            self.roots[root_y] = root_x;
            return true;
        }
    }
}
