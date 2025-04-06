#[derive(Clone)]
pub struct DirectedGraph {
    pub n: usize,
    pub adj: Vec<Vec<usize>>,
}

impl DirectedGraph {
    pub fn new(n: usize) -> Self {
        Self { n, adj: vec![Vec::new(); n] }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
    }

    pub fn find_sccs(&self) -> Vec<Vec<usize>> {
        let postorder = self.postorder();
        let transposed = self.transposed();
        let mut result = Vec::new();
        let mut visited = vec![false; self.n];
        for start in postorder.into_iter().rev() {
            if !visited[start] {
                let mut single_scc = Vec::new();
                transposed.dfs(start, &mut visited, &mut single_scc);
                result.push(single_scc);
            }
        }
        result
    }

    pub fn postorder(&self) -> Vec<usize> {
        let mut result = Vec::new();
        let mut visited = vec![false; self.n];
        for start in 0..self.n {
            if !visited[start] {
                self.dfs(start, &mut visited, &mut result);
            }
        }
        result
    }

    fn dfs(&self, current: usize, visited: &mut [bool], result: &mut Vec<usize>) {
        if visited[current] { return; }
        visited[current] = true;
        for &next in &self.adj[current] {
            self.dfs(next, visited, result);
        }
        result.push(current);
    }

    pub fn transposed(&self) -> Self {
        let mut result = Self::new(self.n);
        for v in 0..self.n {
            for &u in &self.adj[v] {
                result.add_edge(u, v);
            }
        }
        result
    }
}