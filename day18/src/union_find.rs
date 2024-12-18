pub struct UnionFind {
    pub size: usize,
    pub parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            parent: (0..size).collect(),
        }
    }

    pub fn find(&mut self, target: usize) -> usize {
        if self.parent[target] == target { return target }
        let parent = self.find(self.parent[target]);
        self.parent[target] = parent;
        parent  
    }

    pub fn union(&mut self, t1: usize, t2: usize) {
        let p1 = self.find(t1);
        let p2 = self.find(t2);
        if p1 == p2 { return }
        self.parent[p1] = p2;
    }
}
