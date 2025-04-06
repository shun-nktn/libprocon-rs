#[derive(Clone)]
pub struct DisjointSet {
    parents: Vec<Option<usize>>,
    ranks: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![None; n],
            ranks: vec![0; n],
        }
    }

    pub fn find(&mut self, u: usize) -> usize {
        match self.parents[u] {
            None => u,
            Some(parent) => {
                let root = self.find(parent);
                self.parents[u] = Some(root);
                root
            },
        }
    }

    pub fn union(&mut self, u: usize, v: usize) -> usize {
        let rootu = self.find(u);
        let rootv = self.find(v);
        if self.ranks[rootu] < self.ranks[rootv] {
            self.parents[rootu] = Some(rootv);
            rootv
        } else {
            self.parents[rootv] = Some(rootu);
            if self.ranks[rootu] == self.ranks[rootv] {
                self.ranks[rootu] += 1;
            }
            rootu
        }
    }
}