pub trait FenwickCompatible: Copy {
    /// x + zero() == x
    fn zero() -> Self;
    /// x + x.neg() == zero()
    fn neg(self) -> Self;
    /// x.add(y) == y.add(x) && x.add(y).add(z) == x.add(y.add(z))
    fn add(self, rhs: Self) -> Self;
    /// x.sub(y) == y.neg().add(x)
    fn sub(self, rhs: Self) -> Self;
    /// x.scale(n) == (0..n).fold(zero(), |acc, _| acc.add(x))
    fn scale(self, n: usize) -> Self;
    fn add_assign(&mut self, rhs: Self) { *self = self.add(rhs) }
}

impl FenwickCompatible for isize {
    fn zero() -> Self { 0 }
    fn neg(self) -> Self { -self }
    fn add(self, rhs: Self) -> Self { self + rhs }
    fn sub(self, rhs: Self) -> Self { self - rhs }
    fn scale(self, n: usize) -> Self { self * (n as isize) }
}

#[derive(Clone)]
pub struct FenwickTree<T> where 
    T: FenwickCompatible {
    diffs: PrimitiveFenwickTree<T>,
    offsets: PrimitiveFenwickTree<T>,
}

impl<T> FenwickTree<T> where 
    T: FenwickCompatible {
    pub fn new(size: usize) -> Self {
        let diffs = PrimitiveFenwickTree::new(size);
        let offsets = PrimitiveFenwickTree::new(size);
        Self { diffs, offsets }
    }

    pub fn add(&mut self, begin: usize, end: usize, val: T) {
        if begin >= end { return; }
        self.diffs.add(begin, val);
        self.diffs.add(end, val.neg());
        self.offsets.add(begin, val.scale(begin).neg());
        self.offsets.add(end, val.scale(end));
    }

    pub fn sum(&self, begin: usize, end: usize) -> T {
        if begin >= end { return T::zero(); }
        self.sum_until(end).sub(self.sum_until(begin))
    }

    fn sum_until(&self, end: usize) -> T {
        if end == 0 { return T::zero(); }
        let sum = self.diffs.sum(end - 1);
        let offset = self.offsets.sum(end - 1);
        sum.scale(end).add(offset)
    }
}

#[derive(Clone)]
struct PrimitiveFenwickTree<T> where 
    T: FenwickCompatible {
    tree: Vec<T>
}

impl<T> PrimitiveFenwickTree<T> where 
    T: FenwickCompatible {
    fn new(size: usize) -> Self {
        let tree = vec![T::zero(); size];
        Self { tree }
    }

    fn add(&mut self, mut idx: usize, val: T) {
        idx += 1;
        while idx <= self.tree.len() {
            self.tree[idx - 1].add_assign(val);
            idx += idx & idx.wrapping_neg();
        }
    }

    fn sum(&self, mut idx: usize) -> T {
        let mut result = T::zero();
        idx += 1;
        while idx > 0 {
            result.add_assign(self.tree[idx - 1]);
            idx -= idx & idx.wrapping_neg();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let ft = FenwickTree::<isize>::new(0);
        assert_eq!(ft.sum(0, 0), 0);
    }

    #[test]
    fn test_single_update() {
        let mut ft = FenwickTree::<isize>::new(10);
        assert_eq!(ft.sum(0, 10), 0);

        ft.add(0, 5, 10);
        assert_eq!(ft.sum(0, 5), 10 * 5);
        assert_eq!(ft.sum(0, 10), 10 * 5);
        assert_eq!(ft.sum(5, 10), 0);
    }

    #[test]
    fn test_multiple_updates() {
        let mut ft = FenwickTree::<isize>::new(10);

        ft.add(0, 3, 5);
        ft.add(2, 6, 4);
        ft.add(5, 10, 1);

        assert_eq!(ft.sum(0, 1), 5);
        assert_eq!(ft.sum(0, 2), 5 + 5);
        assert_eq!(ft.sum(0, 3), 5 + 5 + 9);
        assert_eq!(ft.sum(0, 5), 5 + 5 + 9 + 4 + 4);
        assert_eq!(ft.sum(0, 6), 5 + 5 + 9 + 4 + 4 + 5);
        assert_eq!(ft.sum(0, 10), 5 + 5 + 9 + 4 + 4 + 5 + 1 + 1 + 1 + 1);

        assert_eq!(ft.sum(2, 6), 22);
        assert_eq!(ft.sum(5, 8), 7);
    }

    #[test]
    fn test_negative_values() {
        let mut ft = FenwickTree::<isize>::new(5);

        ft.add(0, 5, 2);
        ft.add(1, 4, -3);

        assert_eq!(ft.sum(0, 1), 2);
        assert_eq!(ft.sum(0, 2), 2 + (-1));
        assert_eq!(ft.sum(0, 3), 2 + (-1) + (-1));
        assert_eq!(ft.sum(0, 4), 2 + (-1) + (-1) + (-1));
        assert_eq!(ft.sum(0, 5), 2 + (-1) + (-1) + (-1) + 2);
        assert_eq!(ft.sum(1, 4), -3);
    }
}