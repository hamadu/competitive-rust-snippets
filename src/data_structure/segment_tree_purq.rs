trait Monoid {
    fn mul(&self, Self) -> Self;

    fn one() -> Self;
}

struct SegmentTree<T> {
    n: usize,
    data: Vec<T>
}

impl<T: Monoid + Clone> SegmentTree<T> {
    fn new(n: usize, initial: T) -> Self {
        let vs = (n-1).next_power_of_two() << 1;
        SegmentTree { n: n, data: vec![initial; vs] }
    }

    fn new_with(v: &Vec<T>) -> Self {
        let vs = max(4, (v.len()-1).next_power_of_two() << 1);
        let n = v.len();
        let mut data: Vec<T> = vec![T::one(); vs];

        let bottom = vs/2-1;
        for i in 0..n {
            data[bottom+i] = v[i].clone();
        }
        for i in (0..bottom).rev() {
            data[i] = data[i*2+1].mul(data[i*2+2].clone());
        }
        SegmentTree { n: v.len(), data: data }
    }

    fn change(&mut self, idx: usize, new_value: T) {
        let mut pos = self.data.len() / 2 - 1 + idx;
        self.data[pos] = new_value;
        while pos >= 1 {
            let to = (pos - 1) / 2;
            self.data[to] = self.data[to*2+1].mul(self.data[to*2+2].clone());
            pos = to;
        }
    }

    fn range(&self, l: usize, r: usize) -> T {
        self.range2(l, r, 0, 0, self.data.len() / 2)
    }

    fn range2(&self, l: usize, r: usize, idx: usize, segl: usize, segr: usize) -> T {
        if r <= segl || segr <= l {
            return T::one()
        }
        if l <= segl && segr <= r {
            return self.data[idx].clone()
        }
        let med = (segl + segr) / 2;
        self.range2(l, r, idx*2+1, segl, med).mul(self.range2(l, r, idx*2+2, med, segr))
    }
}
