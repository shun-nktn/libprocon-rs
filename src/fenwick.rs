pub struct AbstractFenwick<T> {
    data: Vec<T>,
}

impl<T: Clone> AbstractFenwick<T> {
    pub fn new(init: T, n: usize) -> Self {
        Self { data: vec![init; n] }
    }

    pub fn modify<F>(&mut self, idx: usize, f: F) where
        F: Fn(&mut T),
    {
        let mut idx = idx + 1;
        while idx - 1 < self.data.len() {
            f(&mut self.data[idx - 1]);
            idx += idx & idx.wrapping_neg();
        }
    }

    pub fn prefix_iter(&self, mut idx: usize) -> impl Iterator<Item = &T> + '_ {
        std::iter::from_fn(move || {
            if idx == 0 {
                None
            } else {
                let value = &self.data[idx - 1];
                idx -= idx & idx.wrapping_neg();
                Some(value)
            }
        })
    }
}

pub struct Fenwick {
    diffs: AbstractFenwick<isize>,
    offsets: AbstractFenwick<isize>,
}

impl Fenwick {
    pub fn new(n: usize) -> Self {
        Self {
            diffs: AbstractFenwick::new(0, n),
            offsets: AbstractFenwick::new(0, n)
        }
    }

    pub fn add(&mut self, begin: usize, end: usize, val: isize) {
        self.suffix_add(begin, val);
        self.suffix_add(end, -val);
    }

    fn suffix_add(&mut self, idx: usize, val: isize) {
        self.diffs.modify(idx, |x| *x += val);
        if idx > 0 {
            let offset = (idx as isize - 1) * val;
            self.offsets.modify(idx, |x| *x -= offset);
        }
    }

    pub fn sum(&self, begin: usize, end: usize) -> isize {
        if end <= begin {
            0
        } else if begin == 0 {
            self.prefix_sum(end - 1)
        } else {
            self.prefix_sum(end - 1) - self.prefix_sum(begin - 1)
        }
    }

    fn prefix_sum(&self, idx: usize) -> isize {
        let scale = idx as isize;
        let diff: isize = self.diffs.prefix_iter(idx).sum();
        let offset: isize = self.offsets.prefix_iter(idx).sum();
        scale * diff + offset
    }
}