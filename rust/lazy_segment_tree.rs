#[derive(Clone)]
struct LazySegmentTree<S, F>
where
    S: Clone,
    F: Clone,
{
    n: usize,
    data: Vec<S>,
    lazy_data: Vec<F>,
    op: fn(S, S) -> S,
    e: fn() -> S,
    mapping: fn(F, S) -> S,
    composition: fn(F, F) -> F,
    id: fn() -> F,
}

impl<S, F> LazySegmentTree<S, F>
where
    S: Clone + Copy,
    F: Clone + Copy,
{
    fn new_with_data(
        v: Vec<S>,
        op: fn(S, S) -> S,
        e: fn() -> S,
        mapping: fn(F, S) -> S,
        composition: fn(F, F) -> F,
        id: fn() -> F,
        ) -> Self {
        let n = v.len();
        let mut i = 1;
        while i < n {
            i *= 2;
        }
        let mut data = (0..2*i-1).map(|j| if i - 1 <= j && j < i + n - 1 {
            v[j-i+1]
        } else {
            e()
        }).collect::<Vec<_>>();
        for j in (0..i-1).rev() {
            data[j] = (op)(data[2*j+1], data[2*j+2]);
        }
        return Self {
            n: i,
            data,
            lazy_data: vec![id(); 2*i-1],
            op, e, mapping, composition, id
        };
    }

    fn new(
        n: usize,
        op: fn(S, S) -> S,
        e: fn() -> S,
        mapping: fn(F, S) -> S,
        composition: fn(F, F) -> F,
        id: fn() -> F,
        ) -> Self {
        let mut i = 1;
        while i < n {
            i *= 2;
        }
        let mut data = vec![e(); 2*i-1];
        for j in (0..n-1).rev() {
            data[j] = (op)(data[2*j+1], data[2*j+2]);
        }
        return Self {
            n: i,
            data,
            lazy_data: vec![id(); 2*i-1],
            op, e, mapping, composition, id
        };
    }

    fn update_lazy(&mut self, index: usize) {
        self.data[index] = (self.mapping)(self.lazy_data[index], self.data[index]);
        if index < self.n - 1 {
            self.lazy_data[2*index+1] = (self.composition)(self.lazy_data[2*index+1], self.lazy_data[index]);
            self.lazy_data[2*index+2] = (self.composition)(self.lazy_data[2*index+2], self.lazy_data[index]);
        }
        self.lazy_data[index] = (self.id)();
    }

    fn _query(&mut self, index: usize, l: usize, r: usize, left: usize, right: usize) -> S {
        if r <= left || right <= l {
            return (self.e)();
        }
        self.update_lazy(index);
        if l <= left && right <= r {
            return self.data[index];
        } else {
            let mid = (left + right) / 2;
            let val1 = self._query(2*index+1, l, r, left, mid);
            let val2 = self._query(2*index+2, l, r, mid, right);
            return (self.op)(val1, val2);
        }
    }

    fn query(&mut self, l: usize, r: usize) -> S {
        return self._query(0usize, l, r, 0usize, self.n);
    }

    fn _add_range(&mut self, value: F, index: usize, left: usize, right: usize, l: usize, r: usize) {
        if right <= l || r <= left {
            return;
        }
        self.update_lazy(index);
        if left <= l && r <= right {
            self.lazy_data[index] = (self.composition)(self.lazy_data[index], value);
        } else {
            let mid = (l + r) / 2;
            self._add_range(value, 2*index+1, left, right, l, mid);
            self._add_range(value, 2*index+2, left, right, mid, r);
            let val1 = (self.mapping)(self.lazy_data[2*index+1], self.data[2*index+1]);
            let val2 = (self.mapping)(self.lazy_data[2*index+2], self.data[2*index+2]);
            self.data[index] = (self.op)(val1, val2);
        }
    }

    fn add_range(&mut self, value: F, left: usize, right: usize) {
        self._add_range(value, 0, left, right, 0, self.n);
    }

    fn set(&mut self, index: usize, value: S) {
        let _ = self.query(index, index+1);
        let mut index = self.n + index - 1;
        self.data[index] = value;
        while 0 < index {
            index = (index - 1) / 2;
            self.update_lazy(2*index+1);
            self.update_lazy(2*index+2);
            self.data[index] = (self.op)(self.data[2*index+1], self.data[2*index+2]);
        }
    }
}
