pub struct SegmentTree {
    size: usize,
    values: Vec<usize>, // 1-indexed
    thunks: Vec<Option<usize>>,
}

impl SegmentTree {
    pub fn new(size: usize) -> Self {
        let tree_size = 2 * size.next_power_of_two();
        Self {
            size,
            values: vec![0; tree_size],
            thunks: vec![None; tree_size],
        }
    }

    pub fn range_add(&mut self, begin: usize, end: usize, value: usize) {
        let mut stack = Vec::new();
        stack.push(self.root());
        while let Some(state) = stack.pop() {
            if state.is_disjoint(begin, end) {
                continue;
            }
            self.push(state);
            if state.is_included(begin, end) {
                *self.thunks[state.idx].get_or_insert(0) += value;
                continue;
            }
            stack.push(state.left_child());
            stack.push(state.right_child());
        }
    }

    pub fn range_max(&mut self, begin: usize, end: usize) -> usize {
        let mut current_max = 0;
        let mut stack = Vec::new();
        stack.push(self.root());
        while let Some(state) = stack.pop() {
            if state.is_disjoint(begin, end) {
                continue;
            }
            self.push(state);
            if state.is_included(begin, end) {
                current_max = current_max.max(self.values[state.idx]);
                continue;
            }
            stack.push(state.left_child());
            stack.push(state.right_child());
        }
        current_max
    }

    fn root(&self) -> TraversalState {
        TraversalState { idx: 1, begin: 0, end: self.size }
    }

    fn push(&mut self, state: TraversalState) {
        if let Some(thunk) = self.thunks[state.idx].take() {
            self.values[state.idx] += thunk;
            if !state.is_leaf() {
                *self.thunks[state.left_child().idx].get_or_insert(0) += thunk;
                *self.thunks[state.right_child().idx].get_or_insert(0) += thunk;
            }
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
        Self {
            idx: self.idx * 2,
            begin: self.begin,
            end: (self.begin + self.end) / 2,
        }
    }

    fn right_child(&self) -> Self {
        Self {
            idx: self.idx * 2 + 1,
            begin: (self.begin + self.end) / 2,
            end: self.end
        }
    }
}