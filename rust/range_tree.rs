#[derive(Clone)]
struct RangeTree {
    n: usize,
    data: Vec<Option<usize>>,
}

impl RangeTree {
    fn new(n: usize) -> Self {
        let mut i = 1;
        while i < n {
            i *= 2;
        }
        RangeTree {
            n: i,
            data: vec![None; 2*i-1],
        }
    }

    fn _add_range(&mut self, value: usize, index: usize, left: usize, right: usize, l: usize, r: usize) {
        if self.data[index] != None {
            return;
        }
        if right <= l || r <= left {
            return;
        }
        if left <= l && r <= right {
            self.data[index] = Some(value);
            return;
        }
        let mid = (l + r) / 2;
        self._add_range(value, 2*index+1, left, right, l, mid);
        self._add_range(value, 2*index+2, left, right, mid, r);
    }

    fn add_range(&mut self, value: usize, left: usize, right: usize) {
        self._add_range(value, 0, left, right, 0, self.n);
    }

    fn get(&self, index: usize) -> Option<usize> {
        let mut index = self.n + index - 1;
        while 0 < index {
            if let Some(v) = self.data[index] {
                return Some(v);
            }
            index = (index - 1) / 2;
        }
        return self.data[index];
    }
}


