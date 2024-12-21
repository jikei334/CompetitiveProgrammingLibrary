class UnionFind():
    def __init__(self, n: int):
        self.roots = [-1 for _ in range(n)]

    def root(self, x: int) -> int:
        if self.roots[x] < 0:
            return x
        else:
            self.roots[x] = self.root(self.roots[x])
            return self.roots[x]

    def unite(self, x: int, y: int) -> bool:
        root_x = self.root(x)
        root_y = self.root(y)
        if root_x == root_y:
            return False
        else:
            if self.roots[root_y] < self.roots[root_x]:
                root_x, root_y = root_y, root_x
            self.roots[root_x] += self.roots[root_y]
            self.roots[root_y] = root_x
            return True

    def is_same(self, x: int, y: int) -> bool:
        root_x = self.root(x)
        root_y = self.root(y)
        return root_x == root_y