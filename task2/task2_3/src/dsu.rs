pub struct DSU {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let rank = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        DSU { parent, rank }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
        } else {
            self.parent[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }

    pub fn count(&self) -> usize {
        let mut result = 0;
        for i in 0..self.parent.len() {
            if self.parent[i] == i {
                result += 1;
            }
        }
        result
    }

    pub fn show(&self) {
        println!("parent: {:?}", self.parent);
    }
}
