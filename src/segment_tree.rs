pub trait SegmentTreeCompatible: Copy {
    fn ident() -> Self;
    fn combine(self, rhs: Self) -> Self;
    fn apply(self, rhs: Self) -> Self;
    fn compose(self, rhs: Self) -> Self;
    fn combine_assign(&mut self, rhs: Self) { *self = self.combine(rhs) }
    fn apply_assign(&mut self, rhs: Self) { *self = self.apply(rhs) }
    fn compose_assign(&mut self, rhs: Self) { *self = self.compose(rhs) }
}

#[macro_export]
macro_rules! impl_segment_tree_compatible {
    (
        $type:ty,
        ident = $ident:expr,
        combine = $combine:expr,
        apply = $apply:expr,
        compose = $compose:expr
    ) => {
        impl SegmentTreeCompatible for $type {
            fn ident() -> Self {
                $ident
            }
            fn combine(self, rhs: Self) -> Self {
                $combine(self, rhs)
            }
            fn apply(self, rhs: Self) -> Self {
                $apply(self, rhs)
            }
            fn compose(self, rhs: Self) -> Self {
                $compose(self, rhs)
            }
        }
    };
}

#[derive(Clone)]
pub struct SegmentTree<T> where
    T: SegmentTreeCompatible {
    size: usize,
    values: Vec<T>, // 1-indexed
    thunks: Vec<T>, // 1-indexed
}

impl<T> SegmentTree<T> where 
    T: SegmentTreeCompatible {
    pub fn new(size: usize) -> Self {
        let tree_size = 2 * size.next_power_of_two();
        let values = vec![T::ident(); tree_size];
        let thunks = vec![T::ident(); tree_size];
        Self { size, values, thunks }
    }

    pub fn update(&mut self, begin: usize, end: usize, value: T) {
        let state = self.root();
        self._update(begin, end, value, state);
    }

    fn _update(&mut self, begin: usize, end: usize, value: T, state: TraversalState) {
        if state.is_disjoint(begin, end) { return; }
        if state.is_included(begin, end) {
            self.thunks[state.idx].compose_assign(value);
            self.push(state);
            return;
        }
        self.push(state);
        let left = state.left_child();
        let right = state.right_child();
        self._update(begin, end, value, left);
        self._update(begin, end, value, right);
        self.values[state.idx] = self.values[left.idx].combine(self.values[right.idx]);
    }

    pub fn query(&mut self, begin: usize, end: usize) -> T {
        let state = self.root();
        self._query(begin, end, state)
    }

    fn _query(&mut self, begin: usize, end: usize, state: TraversalState) -> T {
        if state.is_disjoint(begin, end) { return T::ident(); }
        self.push(state);
        if state.is_included(begin, end) {
            return self.values[state.idx];
        }
        let left = self._query(begin, end, state.left_child());
        let right = self._query(begin, end, state.right_child());
        left.combine(right)
    }

    fn root(&self) -> TraversalState {
        TraversalState { idx: 1, begin: 0, end: self.size }
    }

    fn push(&mut self, state: TraversalState) {
        let thunk = self.thunks[state.idx];
        self.values[state.idx].apply_assign(thunk);
        self.thunks[state.idx] = T::ident();
        if !state.is_leaf() {
            self.thunks[state.left_child().idx].compose_assign(thunk);
            self.thunks[state.right_child().idx].compose_assign(thunk);
        }
    }
}

#[derive(Clone, Copy)]
struct TraversalState {
    idx: usize,
    begin: usize,
    end: usize,
}

impl TraversalState {
    fn length(&self) -> usize {
        self.end - self.begin
    }

    fn is_leaf(&self) -> bool {
        self.length() == 1
    }

    fn is_disjoint(&self, begin: usize, end: usize) -> bool {
        self.begin >= end || self.end <= begin
    }

    fn is_included(&self, begin: usize, end: usize) -> bool {
        self.begin >= begin && self.end <= end
    }

    fn left_child(&self) -> Self {
        let idx = self.idx * 2;
        let begin = self.begin;
        let end = (self.begin + self.end) / 2;
        Self { idx, begin, end }
    }

    fn right_child(&self) -> Self {
        let idx = self.idx * 2 + 1;
        let begin = (self.begin + self.end) / 2;
        let end = self.end;
        Self { idx, begin, end }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl_segment_tree_compatible!(
        usize,
        ident = 0,
        combine = |l: usize, r: usize| l.max(r),
        apply   = |l: usize, r: usize| l + r,
        compose = |l: usize, r: usize| l + r
    );

    #[test]
    fn test_update_and_query_whole_range() {
        let size = 5;
        let mut segtree = SegmentTree::<usize>::new(size);

        // Initially, all positions are 0.
        assert_eq!(segtree.query(0, size), 0);

        // Update range [1,4) with +3.
        segtree.update(1, 4, 3);
        // Query whole range should now yield maximum 3.
        assert_eq!(segtree.query(0, size), 3);

        // Query parts of the range.
        // Indices 0 remains 0.
        assert_eq!(segtree.query(0, 1), 0);
        // Indices 1 to 3 now have value 3.
        assert_eq!(segtree.query(1, 4), 3);
        // Index 4 remains 0.
        assert_eq!(segtree.query(4, 5), 0);
    }

    #[test]
    fn test_multiple_updates_and_queries() {
        let size = 5;
        let mut segtree = SegmentTree::<usize>::new(size);

        // Update range [0,3) with +2.
        segtree.update(0, 3, 2);
        // Update range [2,5) with +1.
        segtree.update(2, 5, 1);

        // Now the expected values are:
        // index 0,1: 2
        // index 2: 2 + 1 = 3
        // index 3,4: 1
        // Whole range max should be 3.
        assert_eq!(segtree.query(0, size), 3);
        // Query [0,2): max 2.
        assert_eq!(segtree.query(0, 2), 2);
        // Query [2,3): single element index 2, value 3.
        assert_eq!(segtree.query(2, 3), 3);
        // Query [3,5): max 1.
        assert_eq!(segtree.query(3, 5), 1);
    }

    #[test]
    fn test_single_element_queries() {
        let size = 4;
        let mut segtree = SegmentTree::<usize>::new(size);

        // Perform several updates.
        segtree.update(0, 4, 5);    // All indices +5
        segtree.update(1, 3, 2);    // Indices 1,2 +2
        segtree.update(2, 3, 3);    // Index 2 +3

        // Expected values:
        // index 0: 5
        // index 1: 5 + 2 = 7
        // index 2: 5 + 2 + 3 = 10
        // index 3: 5

        assert_eq!(segtree.query(0, 1), 5);
        assert_eq!(segtree.query(1, 2), 7);
        assert_eq!(segtree.query(2, 3), 10);
        assert_eq!(segtree.query(3, 4), 5);
    }
}
