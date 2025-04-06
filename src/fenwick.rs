pub trait FenwickCompatible {
    type E: Copy;
    fn zero() -> Self::E;
    fn neg(a: Self::E) -> Self::E;
    fn add(a: Self::E, b: Self::E) -> Self::E;
    fn sub(a: Self::E, b: Self::E) -> Self::E { Self::add(a, Self::neg(b)) }
    fn scale(n: usize, a: Self::E) -> Self::E {
        (0..n).fold(Self::zero(), |acc: Self::E, _| Self::add(acc, a))
    }
}

pub struct FenwickTree<T> where 
    T: FenwickCompatible {
    diff: PrimitiveFenwickTree<T>,
    offset: PrimitiveFenwickTree<T>,
}

impl<T> FenwickTree<T> where 
    T: FenwickCompatible {
    pub fn new(size: usize) -> Self {
        let diff = PrimitiveFenwickTree::new(size);
        let offset = PrimitiveFenwickTree::new(size);
        Self { diff, offset }
    }

    pub fn add(&mut self, begin: usize, end: usize, val: T::E) {
        if begin >= end { return; }
        self.diff.add(begin, val);
        self.diff.add(end, T::neg(val));
        self.offset.add(begin, T::scale(begin, T::neg(val)));
        self.offset.add(end, T::scale(end, val));
    }

    pub fn sum(&self, begin: usize, end: usize) -> T::E {
        if begin >= end { return T::zero(); }
        match begin {
            0 => self.sum_until(end),
            _ => T::sub(self.sum_until(end), self.sum_until(begin)),
        }
    }

    fn sum_until(&self, end: usize) -> T::E {
        let sum = self.diff.sum(end - 1);
        let offset = self.offset.sum(end - 1);
        T::add(T::scale(end, sum), offset)
    }
}

struct PrimitiveFenwickTree<T> where 
    T: FenwickCompatible {
    tree: Vec<T::E>
}

impl<T> PrimitiveFenwickTree<T> where 
    T: FenwickCompatible {
    fn new(size: usize) -> Self {
        let val = vec![T::zero(); size];
        Self { tree: val }
    }

    fn add(&mut self, mut idx: usize, val: T::E) {
        idx += 1;
        while idx <= self.tree.len() {
            self.tree[idx - 1] = T::add(self.tree[idx - 1], val);
            idx += idx & idx.wrapping_neg();
        }
    }

    fn sum(&self, mut idx: usize) -> T::E {
        let mut result = T::zero();
        idx += 1;
        while idx > 0 {
            result = T::add(result, self.tree[idx - 1]);
            idx -= idx & idx.wrapping_neg();
        }
        result
    }
}

pub struct Sum<T>(std::marker::PhantomData<T>);

impl<T> FenwickCompatible for Sum<T> where
    T: Copy + TryFrom<usize>
    + std::ops::Neg<Output = T>
    + std::ops::Add<Output = T>
    + std::ops::Sub<Output = T>
    + std::ops::Mul<Output = T>,
    T::Error: std::fmt::Debug {
    type E = T;
    fn zero() -> Self::E { T::try_from(0).unwrap() }
    fn neg(a: Self::E) -> Self::E { - a }
    fn add(a: Self::E, b: Self::E) -> Self::E { a + b }
    fn sub(a: Self::E, b: Self::E) -> Self::E { a - b }
    fn scale(n: usize, a: Self::E) -> Self::E { T::try_from(n).unwrap() * a }
}



#[cfg(test)]
mod tests {
    use super::*;

    /// 基本的な動作確認: サイズ 0 の Fenwick Tree でも panic しないかなど
    #[test]
    fn test_empty() {
        let ft = FenwickTree::<Sum<i32>>::new(0);
        assert_eq!(ft.sum(0, 0), 0);
    }

    /// 単一区間の更新・取得テスト
    /// [0..5) の要素に 10 を加算し、sum(0..5) と sum(0..10) が想定通りか確認
    #[test]
    fn test_single_update() {
        let mut ft = FenwickTree::<Sum<i32>>::new(10);
        assert_eq!(ft.sum(0, 10), 0);

        ft.add(0, 5, 10);
        // [0..5) に 10 が加算されたはず
        // したがって、sum(0..5) = 10 * 5 = 50, sum(5..10) = 0
        assert_eq!(ft.sum(0, 5), 10 * 5);
        assert_eq!(ft.sum(0, 10), 10 * 5);

        // [5..10) は更新していないため 0 のまま
        assert_eq!(ft.sum(5, 10), 0);
    }

    /// 複数区間を更新した場合のテスト
    /// - [0..3) に 5
    /// - [2..6) に 4
    /// - [5..10) に 1
    #[test]
    fn test_multiple_updates() {
        let mut ft = FenwickTree::<Sum<i32>>::new(10);

        ft.add(0, 3, 5);   // 区間 [0..3) = 5
        ft.add(2, 6, 4);   // 区間 [2..6) = 4
        ft.add(5, 10, 1);  // 区間 [5..10) = 1

        // 期待される要素配列 (index 0~9):
        // idx:  0   1   2    3    4    5    6   7   8   9
        // val: [5,  5,  5+4, 4,   4,   4+1, 1,  1,  1,  1 ]
        //      [5,  5,    9,   4,   4,    5,   1,  1,  1,  1 ]
        // 具体的には:
        //  idx=0 => 5
        //  idx=1 => 5
        //  idx=2 => 9
        //  idx=3 => 4
        //  idx=4 => 4
        //  idx=5 => 5
        //  idx=6 => 1
        //  idx=7 => 1
        //  idx=8 => 1
        //  idx=9 => 1

        // sum(0..1) = [0] -> 5
        assert_eq!(ft.sum(0, 1), 5);
        // sum(0..2) = [0,1] -> 5 + 5 = 10
        assert_eq!(ft.sum(0, 2), 10);
        // sum(0..3) = [0,1,2] -> 5 + 5 + 9 = 19
        assert_eq!(ft.sum(0, 3), 19);
        // sum(0..5) = [0,1,2,3,4] -> 5 + 5 + 9 + 4 + 4 = 27
        assert_eq!(ft.sum(0, 5), 27);
        // sum(0..6) = [0..5] + idx=5 -> 27 + 5 = 32
        assert_eq!(ft.sum(0, 6), 32);
        // sum(0..10) = [すべて] -> 32 + idx=6..9 = 1 + 1 + 1 + 1 = 36
        assert_eq!(ft.sum(0, 10), 36);

        // 部分区間テスト
        // sum(2..6) = idx=2,3,4,5 -> 9 + 4 + 4 + 5 = 22
        assert_eq!(ft.sum(2, 6), 22);
        // sum(5..8) = idx=5,6,7 -> 5 + 1 + 1 = 7
        assert_eq!(ft.sum(5, 8), 7);
    }

    /// 負の値を含むテスト
    #[test]
    fn test_negative_values() {
        let mut ft = FenwickTree::<Sum<i32>>::new(5);

        // [0..5) 全体に 2 を加算
        ft.add(0, 5, 2);
        // [1..4) に -3 を加算（部分的に値が引かれるはず）
        ft.add(1, 4, -3);

        // 結果の期待値 (index 0~4):
        // idx=0 => +2
        // idx=1 => +2 + (-3) = -1
        // idx=2 => +2 + (-3) = -1
        // idx=3 => +2 + (-3) = -1
        // idx=4 => +2
        // 合計 -> 2, -1, -1, -1, 2

        assert_eq!(ft.sum(0, 1), 2); // 0 番目のみ
        assert_eq!(ft.sum(0, 2), 2 + (-1)); // [0,1]
        assert_eq!(ft.sum(0, 3), 2 + (-1) + (-1)); // [0,1,2]
        assert_eq!(ft.sum(0, 4), 2 + (-1) + (-1) + (-1)); // [0..3]
        assert_eq!(ft.sum(0, 5), 2 + (-1) + (-1) + (-1) + 2); // [0..4]

        // 部分区間の確認: sum(1..4) = [-1, -1, -1] = -3
        assert_eq!(ft.sum(1, 4), -3);
    }
}
