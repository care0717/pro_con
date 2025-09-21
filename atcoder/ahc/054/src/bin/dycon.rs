use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Copy, Clone)]
pub enum Parent {
    Node(usize), // parent node in the tree
    Path(usize), // path to the root in the forest
    Root,        // root of the tree
}

pub struct Node<T: Path> {
    pub idx: usize,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub parent: Parent,
    pub flipped: bool,
    // for path aggregation:
    pub weight: f64,
    pub path: T,
    // for deletion (the number of edges connected to this node):
    pub degree: usize,
}

impl<T: Path> Node<T> {
    pub fn new(idx: usize, weight: f64) -> Self {
        Node {
            idx,
            left: None,
            right: None,
            parent: Parent::Root,
            flipped: false,
            weight,
            path: T::default(weight, idx),
            degree: 0,
        }
    }

    pub fn flip_children(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> String {
        let parent = match self.parent {
            Parent::Node(idx) => format!("Node({idx})"),
            Parent::Path(idx) => format!("Path({idx})"),
            Parent::Root => "Root".to_string(),
        };
        format!(
            "Node {{ idx: {}, left: {:?}, right: {:?}, parent: {parent:?}}}",
            self.idx, self.left, self.right
        )
    }
}

pub struct Index {
    time_id: usize,
    deleted_ids: Vec<usize>, // maybe use a set instead?
}

impl Index {
    pub fn new() -> Self {
        Self {
            time_id: 0,
            deleted_ids: Vec::new(),
        }
    }

    pub fn insert(&mut self) -> usize {
        if !self.deleted_ids.is_empty() {
            return self.deleted_ids.pop().unwrap();
        }
        self.time_id += 1;
        self.time_id - 1
    }

    pub fn delete(&mut self, id: usize) {
        assert!(id < self.time_id, "Invalid deletion");
        self.deleted_ids.push(id);
    }
}

pub struct Forest<P: Path> {
    nodes: Vec<Node<P>>,
    index: Index,
}

impl<P: Path> Forest<P> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            index: Index::new(),
        }
    }

    pub fn create_node(&mut self, weight: f64) -> usize {
        let idx = self.index.insert();
        if idx < self.nodes.len() {
            self.nodes[idx] = Node::new(idx, weight);
            return idx;
        }
        self.nodes.push(Node::new(idx, weight));
        idx
    }

    pub fn delete_node(&mut self, node_idx: usize) {
        assert!(
            self.nodes[node_idx].degree == 0,
            "Invalid deletion: tree contains more than one node."
        );
        self.index.delete(node_idx);
    }

    #[inline]
    pub fn set_right(&mut self, node_idx: usize, right_idx: usize) {
        assert!(
            self.nodes[node_idx].right.is_none(),
            "set_right: node_idx already has a right child"
        );
        self.nodes[node_idx].right = Some(right_idx);
        self.nodes[right_idx].parent = Parent::Node(node_idx);
    }

    #[inline]
    pub fn set_left(&mut self, node_idx: usize, left_idx: usize) {
        assert!(
            self.nodes[node_idx].left.is_none(),
            "set_left: node_idx already has a left child"
        );
        self.nodes[node_idx].left = Some(left_idx);
        self.nodes[left_idx].parent = Parent::Node(node_idx);
        self.nodes[node_idx].degree += 1;
        self.nodes[left_idx].degree += 1;
    }

    #[inline]
    pub fn cut_left(&mut self, node_idx: usize) {
        assert!(
            self.nodes[node_idx].left.is_some(),
            "cut_left: node_idx does not have a left child"
        );
        let left = self.nodes[node_idx].left.unwrap();
        self.nodes[node_idx].left = None;
        self.nodes[left].parent = Parent::Root;
        self.nodes[node_idx].degree -= 1;
        self.nodes[left].degree -= 1;
    }

    #[inline]
    pub fn parent_of(&self, node_idx: usize) -> Option<usize> {
        if let Parent::Node(parent_idx) = self.nodes[node_idx].parent {
            Some(parent_idx)
        } else {
            None
        }
    }

    #[inline]
    pub fn path_parent_of(&self, node_idx: usize) -> Option<usize> {
        if let Parent::Path(parent_idx) = self.nodes[node_idx].parent {
            Some(parent_idx)
        } else {
            None
        }
    }

    #[inline]
    pub fn left_of(&self, node_idx: usize) -> Option<usize> {
        self.nodes[node_idx].left
    }

    #[inline]
    pub fn right_of(&self, node_idx: usize) -> Option<usize> {
        self.nodes[node_idx].right
    }

    #[inline]
    pub fn aggregated_path_of(&self, node_idx: usize) -> P {
        self.nodes[node_idx].path
    }

    // Unflips the subtree rooted at `node_idx`, swapping the left and right children.
    // The children's `flipped` flag is also toggled to propogate the change down the tree.
    pub fn normalize(&mut self, node_idx: usize) {
        if self.nodes[node_idx].flipped {
            self.nodes[node_idx].flip_children();
            self.nodes[node_idx].flipped = false;
            if let Some(left_child) = self.nodes[node_idx].left {
                self.nodes[left_child].flipped ^= true;
            }
            if let Some(right_child) = self.nodes[node_idx].right {
                self.nodes[right_child].flipped ^= true;
            }
        }
    }

    // Updates the path aggregate information for the subtree rooted at `node_idx`.
    pub fn update(&mut self, node_idx: usize) {
        self.nodes[node_idx].path = P::default(self.nodes[node_idx].weight, node_idx);
        if let Some(left_child) = self.nodes[node_idx].left {
            let left_path = self.nodes[left_child].path;
            self.nodes[node_idx].path.aggregate(left_path);
        }
        if let Some(right_child) = self.nodes[node_idx].right {
            let right_path = self.nodes[right_child].path;
            self.nodes[node_idx].path.aggregate(right_path);
        }
    }

    pub fn remove_preferred_child(&mut self, node_idx: usize) {
        if let Some(right_idx) = self.nodes[node_idx].right {
            self.nodes[node_idx].right = None;
            self.nodes[right_idx].parent = Parent::Path(node_idx);
            self.update(node_idx);
        }
    }

    pub fn flip(&mut self, node_idx: usize) {
        self.nodes[node_idx].flipped ^= true;
        self.normalize(node_idx);
    }

    fn rotate_left(&mut self, node_idx: usize) {
        assert!(
            self.nodes[node_idx].right.is_some(),
            "rotate_left: node_idx does not have a right child"
        );

        let right_child = self.nodes[node_idx].right.unwrap();
        if let Parent::Node(parent_idx) = self.nodes[node_idx].parent {
            if self.nodes[parent_idx].left == Some(node_idx) {
                self.nodes[parent_idx].left = Some(right_child);
            } else {
                self.nodes[parent_idx].right = Some(right_child);
            }
        }

        self.nodes[node_idx].right = self.nodes[right_child].left;
        self.nodes[right_child].left = Some(node_idx);
        self.nodes[right_child].parent = self.nodes[node_idx].parent;
        self.nodes[node_idx].parent = Parent::Node(right_child);

        if let Some(new_right_child) = self.nodes[node_idx].right {
            self.nodes[new_right_child].parent = Parent::Node(node_idx);
        }
    }

    fn rotate_right(&mut self, node_idx: usize) {
        assert!(
            self.nodes[node_idx].left.is_some(),
            "rotate_right: node_idx does not have a left child"
        );

        let left_child = self.nodes[node_idx].left.unwrap();
        if let Parent::Node(parent_idx) = self.nodes[node_idx].parent {
            if self.nodes[parent_idx].left == Some(node_idx) {
                self.nodes[parent_idx].left = Some(left_child);
            } else {
                self.nodes[parent_idx].right = Some(left_child);
            }
        }

        self.nodes[node_idx].left = self.nodes[left_child].right;
        self.nodes[left_child].right = Some(node_idx);
        self.nodes[left_child].parent = self.nodes[node_idx].parent;
        self.nodes[node_idx].parent = Parent::Node(left_child);

        if let Some(new_left_child) = self.nodes[node_idx].left {
            self.nodes[new_left_child].parent = Parent::Node(node_idx);
        }
    }

    // Rotates the parent of `node_idx` to the right or left, depending on the relationship between.
    fn rotate(&mut self, node_idx: usize) {
        assert!(
            matches!(self.nodes[node_idx].parent, Parent::Node(_)),
            "rotate: node_idx does not have a parent"
        );

        if let Parent::Node(parent_idx) = self.nodes[node_idx].parent {
            self.normalize(parent_idx);
            self.normalize(node_idx);
            if self.nodes[parent_idx].left == Some(node_idx) {
                self.rotate_right(parent_idx);
            } else {
                self.rotate_left(parent_idx);
            }
            self.update(parent_idx);
        }
    }

    pub fn splay(&mut self, node_idx: usize) {
        while let Parent::Node(parent_idx) = self.nodes[node_idx].parent {
            if let Parent::Node(grandparent_idx) = self.nodes[parent_idx].parent {
                if (self.nodes[grandparent_idx].left == Some(parent_idx))
                    == (self.nodes[parent_idx].left == Some(node_idx))
                {
                    // zig-zig (same direction):
                    self.rotate(parent_idx);
                } else {
                    // zig-zag:
                    self.rotate(node_idx);
                }
            }
            // zig
            self.rotate(node_idx);
        }
        self.normalize(node_idx);
        self.update(node_idx);
    }
}

pub trait Path: Copy + Clone {
    fn default(weight: f64, index: usize) -> Self;
    fn aggregate(&mut self, other: Self);
}

#[derive(Copy, Clone)]
pub struct FindMax {
    pub idx: usize,
    pub weight: f64,
}

impl Path for FindMax {
    fn default(weight: f64, index: usize) -> Self {
        FindMax { idx: index, weight }
    }

    fn aggregate(&mut self, other: Self) {
        if other.weight > self.weight {
            self.weight = other.weight;
            self.idx = other.idx;
        }
    }
}

#[derive(Copy, Clone)]
pub struct FindMin {
    pub idx: usize,
    pub weight: f64,
}

impl Path for FindMin {
    fn default(weight: f64, index: usize) -> Self {
        FindMin { idx: index, weight }
    }

    fn aggregate(&mut self, other: Self) {
        if other.weight < self.weight {
            self.weight = other.weight;
            self.idx = other.idx;
        }
    }
}

#[derive(Copy, Clone)]
pub struct FindSum {
    pub sum: f64,
}

impl Path for FindSum {
    fn default(weight: f64, _: usize) -> Self {
        FindSum { sum: weight }
    }

    fn aggregate(&mut self, other: Self) {
        self.sum += other.sum;
    }
}

pub struct LinkCutTree<P: Path> {
    forest: Forest<P>,
}

impl<P: Path> LinkCutTree<P> {
    /// Creates a new empty link-cut tree.
    #[must_use]
    pub fn new() -> Self {
        Self {
            forest: Forest::new(),
        }
    }

    pub fn make_tree(&mut self, weight: f64) -> usize {
        self.forest.create_node(weight)
    }

    #[must_use]
    pub fn extend_forest(&mut self, weights: &[f64]) -> Vec<usize> {
        weights
            .iter()
            .map(|&weight| self.make_tree(weight))
            .collect()
    }

    /// Delete a tree with a single node with the given id.
    ///
    /// # Panics
    ///
    /// Panics if the tree contains more than one node.
    pub fn remove_tree(&mut self, idx: usize) {
        self.forest.delete_node(idx);
    }

    /// Constructs a path from a node to the root of the tree.
    fn access(&mut self, v: usize) {
        self.forest.splay(v);
        self.forest.remove_preferred_child(v);

        while let Some(path_idx) = self.forest.path_parent_of(v) {
            self.forest.splay(path_idx);
            self.forest.remove_preferred_child(path_idx);

            self.forest.set_right(path_idx, v);
            self.forest.splay(v); // just a rotation
        }
    }

    /// Makes v the root of its represented tree by flipping the path from v to the root.
    fn reroot(&mut self, v: usize) {
        self.access(v);
        self.forest.flip(v);
    }

    pub fn connected(&mut self, v: usize, w: usize) -> bool {
        v == w || self.findroot(v) == self.findroot(w)
    }

    pub fn link(&mut self, v: usize, w: usize) -> bool {
        self.reroot(v);
        self.access(w);
        // if access(w) messed with the root of the tree, then v and w are connected:
        if self.forest.parent_of(v).is_some() || v == w {
            return false;
        }
        // v is the root of its represented tree:
        self.forest.set_left(v, w);
        true
    }

    pub fn linked(&mut self, v: usize, w: usize) -> bool {
        self.reroot(v);
        self.access(w);
        self.forest.left_of(w) == Some(v) && self.forest.right_of(v).is_none()
    }

    pub fn cut(&mut self, v: usize, w: usize) -> bool {
        if !self.linked(v, w) {
            return false;
        }
        self.forest.cut_left(w);
        true
    }

    pub fn path(&mut self, v: usize, w: usize) -> P {
        self.reroot(v);
        self.access(w);
        if self.forest.parent_of(v).is_none() && v != w {
            return P::default(f64::INFINITY, usize::MAX);
        }
        self.forest.aggregated_path_of(w)
    }

    /// Finds the root of the tree that the query node is in.
    pub fn findroot(&mut self, v: usize) -> usize {
        self.access(v);
        let mut root = v;
        while let Some(left) = self.forest.left_of(root) {
            root = left;
        }
        self.forest.splay(root); // fast access to the root next time
        root
    }
}

impl Default for LinkCutTree<FindMax> {
    fn default() -> Self {
        Self::new()
    }
}

struct DynamicConnectivity {
    n: usize,
    levels: usize,
    forests: Vec<LinkCutTree<FindMax>>,
    non_tree_edges: Vec<HashSet<(usize, usize)>>,
    edge_level: HashMap<(usize, usize), usize>,
}

impl DynamicConnectivity {
    fn new(n: usize) -> Self {
        let levels = (n as f64).log2().ceil() as usize + 1;
        let mut forests = Vec::new();

        for _ in 0..levels {
            let mut lctree = LinkCutTree::default();
            for i in 0..n {
                lctree.make_tree(0.0);
            }
            forests.push(lctree);
        }
        let non_tree_edges = vec![HashSet::new(); levels];
        let edge_level = HashMap::new();
        Self {
            n,
            levels,
            forests,
            non_tree_edges,
            edge_level,
        }
    }

    fn connected(&mut self, u: usize, v: usize) -> bool {
        self.forests[0].connected(u, v)
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        let key = (u.min(v), u.max(v));
        if !self.forests[0].connected(u, v) {
            self.forests[0].link(u, v);
            self.edge_level.insert(key, 0);
        } else {
            self.non_tree_edges[0].insert(key);
            self.edge_level.insert(key, 0);
        }
    }

    fn remove_edge(&mut self, u: usize, v: usize) {
        let key = (u.min(v), u.max(v));
        if let Some(level) = self.edge_level.remove(&key) {
            if self.non_tree_edges[level].remove(&key) {
                return;
            }
            self.forests[level].cut(u, v);
            self.find_replacement(u, v, level);
        }
    }

    fn find_replacement(&mut self, u: usize, _v: usize, mut level: usize) {
        while level < self.levels {
            let mut candidate: Option<(usize, usize)> = None;
            let component = self.get_component(u, level);
            for &edge in &self.non_tree_edges[level] {
                let (a, b) = edge;
                if component.contains(&a) && !component.contains(&b)
                    || component.contains(&b) && !component.contains(&a)
                {
                    candidate = Some(edge);
                    break;
                }
            }
            if let Some(edge) = candidate {
                self.non_tree_edges[level].remove(&edge);
                let (a, b) = edge;
                self.forests[level].link(a, b);
                self.edge_level.insert(edge, level);
                break;
            } else {
                // 昇格
                let promoted: Vec<_> = self.non_tree_edges[level].drain().collect();
                for e in promoted {
                    self.non_tree_edges[level + 1].insert(e);
                    self.edge_level.insert(e, level + 1);
                }
                level += 1;
            }
        }
    }

    fn get_component(&mut self, u: usize, level: usize) -> HashSet<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(u);
        while let Some(node) = queue.pop_front() {
            if !visited.insert(node) {
                continue;
            }
            for v in 0..self.n {
                if self.forests[level].connected(node, v) {
                    queue.push_back(v);
                }
            }
        }
        visited
    }
}

fn main() {
    let mut dc = DynamicConnectivity::new(6);
    dc.add_edge(0, 1);
    dc.add_edge(1, 2);
    dc.add_edge(2, 3);
    println!("0-3 connected? {}", dc.connected(0, 3));
    dc.remove_edge(1, 2);
    println!("0-3 connected after remove 1-2? {}", dc.connected(0, 3));
}
