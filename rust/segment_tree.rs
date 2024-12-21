#[derive(Clone)]
struct SegmentTree<T>
where
    T: Clone,
{
    n: usize,
    data: Vec<T>,
    cmp: fn(T, T) -> T,
}

impl<T> SegmentTree<T>
where
    T: Clone + Copy + std::cmp::PartialEq,
{
    fn new(n: usize, v: T, cmp: fn(T, T) -> T) -> Self {
        let mut i = 1;
        while i < n {
            i *= 2;
        }
        let n = i;
        let mut data = vec![v; 2*i-1];
        for i in (0..n-1).rev() {
            data[i] = cmp(data[2*i+1], data[2*i+2]);
        }
        SegmentTree {
            n,
            data,
            cmp,
        }
    }

    fn set(&mut self, index: usize, value: T) {
        let mut index = self.n + index - 1;
        self.data[index] = value;
        while 0 < index {
            index = (index - 1) / 2;
            self.data[index] = (self.cmp)(self.data[2*index+1], self.data[2*index+2]);
        }
    }

    fn _query(&self, index: usize, l: usize, r: usize, left: usize, right: usize) -> Option<T> {
        if r <= left || right <= l {
            return None;
        } else if l <= left && right <= r {
            return Some(self.data[index]);
        } else {
            let mid = (left + right) / 2;
            let v1 = self._query(2*index+1, l, r, left, mid);
            let v2 = self._query(2*index+2, l, r, mid, right);
            if v1 == None {
                return v2;
            } else if v2 == None {
                return v1;
            } else {
                return Some((self.cmp)(v1.unwrap(), v2.unwrap()));
            }
        }

    }

    fn query(&self, l: usize, r: usize) -> T {
        return self._query(0, l, r, 0, self.n).unwrap();
    }
}
