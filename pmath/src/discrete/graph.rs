//! Graphs and graph algorithms.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::rc::Rc;
use num_traits::{Bounded, ConstZero};

/// A struct representing a directed graph with weighted non-negative edges.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Graph<T, U>
where
    T: Eq + Hash
{
    id_counter: usize,
    freed_ids: Vec<usize>,
    vertex2id: HashMap<Rc<Vertex<T>>, usize>,
    id2vertex: HashMap<usize, Rc<Vertex<T>>>,
    adj_list: HashMap<usize, HashMap<usize, U>>,
}
impl<T, U> Graph<T, U>
where
    T: Eq + Hash + Clone,
    U: ConstZero + Copy + PartialOrd + Bounded,
{
    /// Constructs a new empty `Graph`.
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Constructs a new `Graph` with a specified capacity for the number of vertices.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            id_counter: 0,
            freed_ids: Vec::with_capacity(capacity),
            vertex2id: HashMap::with_capacity(capacity),
            id2vertex: HashMap::with_capacity(capacity),
            adj_list: HashMap::with_capacity(capacity),
        }
    }

    /// Sets the edge between two vertices.
    pub fn set_edge(&mut self, vertex1: &Vertex<T>, vertex2: &Vertex<T>, value: U) {
        if value < U::ZERO {
            panic!("Edge value must be non-negative.");
        }
        let id1 = match self.vertex2id.get(vertex1) {
            Some(id) => *id,
            None => panic!("vertex1 not present in the graph."),
        };
        let id2 = match self.vertex2id.get(vertex2) {
            Some(id) => *id,
            None => panic!("vertex2 not present in the graph."),
        };

        // this is safe because vertex1 is present in the graph, therefore id1 must be present
        // in the adjacency list
        let edges1 = unsafe { self.adj_list.get_mut(&id1).unwrap_unchecked() };

        if value == U::ZERO {
            edges1.remove(&id2);
        } else {
            edges1.insert(id2, value);
        }
    }

    /// Sets the edge between two vertices in both directions.
    pub fn set_edge_undirected(&mut self, vertex1: &Vertex<T>, vertex2: &Vertex<T>, value: U) {
        self.set_edge(vertex1, vertex2, value);
        self.set_edge(vertex2, vertex1, value);
    }

    /// Gets the edge between two vertices. zero if no edge exists.
    pub fn edge(&self, vertex1: &Vertex<T>, vertex2: &Vertex<T>) -> U {
        let id1 = match self.vertex2id.get(vertex1) {
            Some(id) => *id,
            None => panic!("vertex1 not present in the graph."),
        };
        let id2 = match self.vertex2id.get(vertex2) {
            Some(id) => *id,
            None => panic!("vertex2 not present in the graph."),
        };

        // this is safe because vertex1 is present in the graph, therefore id1 must be present
        // in the adjacency list
        let edges1 = unsafe { self.adj_list.get(&id1).unwrap_unchecked() };

        *edges1.get(&id2).unwrap_or(&U::ZERO)
    }

    /// Adds a vertex to the graph if it is not already present.
    pub fn add(&mut self, vertex: &Vertex<T>) {
        if self.contains(vertex) {
            return;
        }
        let id = if let Some(id) = self.freed_ids.pop() {
            id
        } else {
            let id = self.id_counter;
            if id == usize::MAX {
                panic!("Maximum number of vertices reached.");
            }
            self.id_counter += 1;
            id
        };
        let vertex = Rc::new(vertex.clone());
        self.vertex2id.insert(vertex.clone(), id);
        self.id2vertex.insert(id, vertex);
        self.adj_list.insert(id, HashMap::new());
    }

    /// Removes a vertex from the graph if it exists.
    pub fn remove(&mut self, vertex: &Vertex<T>) {
        if let Some(id) = self.vertex2id.remove(vertex) {
            self.id2vertex.remove(&id);
            self.adj_list.remove(&id);
            for edges in self.adj_list.values_mut() {
                edges.remove(&id);
            }
            self.freed_ids.push(id);
        }
    }

    /// Checks if the graph contains a vertex.
    pub fn contains(&self, vertex: &Vertex<T>) -> bool {
        self.vertex2id.contains_key(vertex)
    }

    /// Returns the number of vertices in the graph.
    pub fn len(&self) -> usize {
        self.vertex2id.len()
    }

    /// Gets the iterator over the vertices in the graph.
    pub fn vertices(&self) -> impl Iterator<Item = &Vertex<T>> {
        self.vertex2id.keys().map(|vertex| vertex.as_ref())
    }

    /// Finds the shortest Hamiltonian cycle in the graph.
    /// Returns a tuple containing the minimum cost and the vertices in the cycle.
    /// Since this is a cycle, vertices can be rotated to start from any vertex.
    pub fn hamiltonian_cycle_min(&self) -> (U, Vec<&Vertex<T>>) {
        unimplemented!()
    }

    /// Finds the longest Hamiltonian cycle in the graph.
    /// Returns a tuple containing the maximum cost and the vertices in the cycle.
    /// Since this is a cycle, vertices can be rotated to start from any vertex.
    /// The direction of the cycle is from lower indices to higher indices.
    pub fn hamiltonian_cycle_max(&self) -> (U, Vec<&Vertex<T>>) {
        if self.adj_list.len() < 2 {
            panic!("The graph must contain at least 2 vertices.");
        }

        // define node structure used in the algorithm
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
        struct Node {
            max_cost: isize,   // maximum cost for the whole cycle following this node
            path: Vec<Vertex>, // path from the starting node to this one
        }
        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Node {
            fn cmp(&self, other: &Self) -> Ordering {
                self.max_cost.cmp(&other.max_cost)
            }
        }

        // initialize cycle and maximum cost
        let mut max_cycle = Vec::new();
        let mut max_cost = isize::MIN;

        // find maximum edge weight from every vertex
        let max_edges = self
            .adj_list
            .iter()
            .map(|(key, value)| {
                (
                    *key,
                    value
                        .iter()
                        .map(|edge| edge.1)
                        .max()
                        .expect("Invalid graph! (vertex with no edges)"),
                )
            })
            .collect::<HashMap<_, _>>();

        // priority queue
        // nodes with bigger max_cost are popped first
        let mut queue = BinaryHeap::new();

        // take random vertex as starting point
        // it doesn't matter which vertex is chosen as the starting point
        // because the cycle can be rotated to start from any vertex
        let start_node = Node {
            max_cost: max_edges.values().sum(),
            path: vec![*self.adj_list.keys().next().unwrap()],
        };

        // add starting node to the queue
        queue.push(start_node);

        // process nodes until all are processed
        // or the max_cost for the popped node is less than absolute max_cost
        // (all other nodes also have smaller max_cost since this is a priority queue)
        while let Some(mut node) = queue.pop() {
            if node.max_cost < max_cost {
                break;
            }

            if node.path.len() == self.adj_list.len() {
                // if node contains a path with the number of vertices equal to the total number of vertices,
                // process the final edge
                // (last vertex -> first vertex) if there is one,
                // and update max_cost and max_cycle if necessary

                let first_vertex = node.path.first().unwrap();
                let last_vertex = node.path.last().unwrap();
                node.max_cost -= max_edges[last_vertex];
                match self.adj_list[last_vertex]
                    .iter()
                    .find(|(id, _)| id == first_vertex)
                {
                    None => continue,
                    Some(edge) => node.max_cost += edge.1,
                }

                if node.max_cost > max_cost {
                    max_cost = node.max_cost;
                    max_cycle = node.path;
                }
            } else {
                // if node contains path with fewer vertices than total,
                // consider all possible moves to next vertex along edge
                // (if that vertex isn't already visited, in the nodes path)
                // for each possible move, clone node, update max_cost and path, add to queue
                let last_vertex = node.path.last().unwrap();
                for (other, weight) in &self.adj_list[last_vertex] {
                    if !node.path.contains(other) {
                        let mut new_node = node.clone();
                        new_node.max_cost -= max_edges[last_vertex];
                        new_node.max_cost += weight;
                        if new_node.max_cost > max_cost {
                            new_node.path.push(*other);
                            queue.push(new_node)
                        }
                    }
                }
            }
        }

        // if max_cost is still at its initial value, no cycle was found
        // else return the maximum cost and the cycle
        if max_cost == isize::MIN {
            panic!("No cycle found!");
        } else {
            (max_cost, max_cycle)
        }
    }

    fn hamiltonian_cycle<X, Y, Z>(&self, minimum: bool, weight: X, neighbours: Y) -> Option<(U, Vec<&Vertex<T>>)>
    where
        X: Fn(&usize, &usize) -> Option<U>,
        Y: Fn(&usize) -> Z,
        Z: IntoIterator<Item = (usize, U)>,
    {
        #[derive(Eq, PartialEq)]
        struct Node<V> {
            value: usize, // vertex id
            cost: V,      // min/max possible cost for the full cycle following this node
            prev_node: Option<Rc<Node<V>>>,
        }
        impl<V> Node<V> {
            fn new(value: usize, cost: V, prev_node: Option<Rc<Node<V>>>) -> Self {
                Self { value, cost, prev_node }
            }
        }
        impl<V: Ord> PartialOrd for Node<V> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl<V: Ord> Ord for Node<V> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.cost.cmp(&other.cost)
            }
        }

        // initialize the best node and the best cost
        let best_cost_init = if minimum { U::max_value() } else { U::min_value() };
        let mut best_node = None;
        let mut best_cost = best_cost_init;

        // best edges optimization, minimum edge weight of future nodes is added to the cost of
        // the current node, so that cost can hopefully be pruned earlier as it is impossible
        // for the total cost to be less than the sum of minimum edge weights of future nodes
        let best_edges = self.id2vertex.keys()
            .map(|id| {
                let mut best = best_cost_init;
                if minimum {
                    for (_, weight) in neighbours(id).into_iter() {
                        if weight < best {
                            best = weight;
                        }
                    }
                } else {
                    for (_, weight) in neighbours(id).into_iter() {
                        if weight > best {
                            best = weight;
                        }
                    }
                }
                (*id, best)
            })
            .collect::<HashMap<_, _>>();

        // priority queue (max popped first)
        let mut queue = BinaryHeap::new();

        // take random vertex as starting point
        // it doesn't matter which vertex is chosen as starting point
        // because the cycle can be rotated to start from any vertex
        let start_node = Node::new(0, 0, None);

        // add starting node to the queue
        queue.push(start_node);

        // process nodes until all are processed
        // or the min_cost for popped node is greater than absolute min_cost
        // (all other nodes also have bigger min_cost since this is priority queue)
        while let Some(mut node) = queue.pop() {
            if node.min_cost > min_cost {
                break;
            }

            if node.path.len() == self.adj_list.len() {
                // if node contains path with the number of vertices equal to total number of vertices,
                // process the final edge (last vertex -> first vertex), if there is one, and update min_cost and min_cycle if necessary

                let first_vertex = node.path.first().unwrap();
                let last_vertex = node.path.last().unwrap();
                node.min_cost -= min_edges[last_vertex];
                match self.adj_list[last_vertex]
                    .iter()
                    .find(|(id, _)| id == first_vertex)
                {
                    None => continue,
                    Some(edge) => node.min_cost += edge.1,
                }

                if node.min_cost < min_cost {
                    min_cost = node.min_cost;
                    min_cycle = node.path;
                }
            } else {
                // if a node contains a path with fewer vertices than total,
                // consider all possible moves to the next vertex along the edge
                // (if that vertex isn't already visited, in the nodes path)
                // for each possible move, clone node, update min_cost and path, add to queue
                let last_vertex = node.path.last().unwrap();
                for (other, weight) in &self.adj_list[last_vertex] {
                    if !node.path.contains(other) {
                        let mut new_node = node.clone();
                        new_node.min_cost -= min_edges[last_vertex];
                        new_node.min_cost += weight;
                        if new_node.min_cost < min_cost {
                            new_node.path.push(*other);
                            queue.push(new_node)
                        }
                    }
                }
            }
        }

        // if min_cost is still at its initial value, no cycle was found
        // else return the minimum cost and the cycle
        if best_cost == best_cost_init {
            panic!("No cycle found!");
        } else {
            (min_cost, min_cycle)
        }
    }

    /// Finds the shortest Hamiltonian path in the graph.
    /// Returns a tuple containing the minimum cost and the vertices in the path.
    /// The direction of the path is from lower indices to higher indices.
    pub fn hamiltonian_path_min(&self) -> (U, Vec<&Vertex<T>>) {
        if self.adj_list.len() < 2 {
            panic!("The graph must contain at least 2 vertices.");
        }

        // existing vertices
        let vertices = self.vertices().collect::<Vec<_>>();

        // add new vertex
        let added_vertex = self.new_vertex();

        // set all edges to/from the added vertex to 0
        for vertex in &vertices {
            self.adj_list
                .get_mut(vertex)
                .unwrap()
                .push((added_vertex, 0));
        }
        self.adj_list
            .get_mut(&added_vertex)
            .unwrap()
            .extend(vertices.iter().map(|vertex| (*vertex, 0)));

        // find minimum hamiltonian cycle
        let (min_cost, mut min_path) = self.hamiltonian_cycle_min();

        // since added_vertex edges are 0, min_cost is correct
        // min_path is actually min_cycle that needs to be transformed into min_path
        // added_vertex must be removed from it, and adjacent vertices must be first and last vertices in the path

        // find current position of the added_vertex
        let added_vertex_pos = min_path
            .iter()
            .position(|vertex| vertex == &added_vertex)
            .unwrap();

        // rotate min_path so that the added_vertex is at index 0
        min_path.rotate_left(added_vertex_pos);

        // remove added_vertex
        min_path.remove(0);

        // remove added_vertex from the graph
        self.remove_vertex(added_vertex);

        // return min_cost and min_path
        (min_cost, min_path)
    }

    /// Finds the longest Hamiltonian path in the graph.
    /// Returns a tuple containing the maximum cost and the vertices in the path.
    /// The direction of the path is from lower indices to higher indices.
    pub fn hamiltonian_path_max(&self) -> (U, Vec<&Vertex<T>>) {
        if self.adj_list.len() < 2 {
            panic!("The graph must contain at least 2 vertices.");
        }

        // existing vertices
        let vertices = self.vertices().collect::<Vec<_>>();

        // add new vertex
        let added_vertex = self.new_vertex();

        // set all edges to/from the added vertex to 0
        for vertex in &vertices {
            self.adj_list
                .get_mut(vertex)
                .unwrap()
                .push((added_vertex, 0));
        }
        self.adj_list
            .get_mut(&added_vertex)
            .unwrap()
            .extend(vertices.iter().map(|vertex| (*vertex, 0)));

        // find maximum hamiltonian cycle
        let (max_cost, mut max_path) = self.hamiltonian_cycle_max();

        // since added_vertex edges are 0, max_cost is correct
        // max_path is actually max_cycle that needs to be transformed into max_path
        // added_vertex must be removed from it, and adjacent vertices must be first and last vertices in the path

        // find the current position of the added_vertex
        let added_vertex_pos = max_path
            .iter()
            .position(|vertex| vertex == &added_vertex)
            .unwrap();

        // rotate max_path so that the added_vertex is at index 0
        max_path.rotate_left(added_vertex_pos);

        // remove added_vertex
        max_path.remove(0);

        // remove added_vertex from the graph
        self.remove_vertex(added_vertex);

        // return min_cost and min_path
        (max_cost, max_path)
    }

    /// Finds the shortest Hamiltonian path in the graph with fixed ends.
    /// Note that the ends are fixed, but not the direction of the path.
    /// Returns a tuple containing the minimum cost and the vertices in the path.
    /// The direction of the path is from lower indices to higher indices.
    pub fn hamiltonian_path_min_fixed_ends(
        &self,
        end1: &Vertex<T>,
        end2: &Vertex<T>,
    ) -> (U, Vec<&Vertex<T>>) {
        if self.adj_list.len() < 2 {
            panic!("The graph must contain at least 2 vertices.");
        }
        if !self.adj_list.contains_key(&end1) {
            panic!("end1 not present in the graph.");
        }
        if !self.adj_list.contains_key(&end2) {
            panic!("end2 not present in the graph.");
        }

        // add new vertex
        let added_vertex = self.new_vertex();

        // set edges between start and added_vertex, added_vertex and end, to 0
        self.adj_list
            .get_mut(&added_vertex)
            .unwrap()
            .push((end1, 0));
        self.adj_list
            .get_mut(&end1)
            .unwrap()
            .push((added_vertex, 0));
        self.adj_list
            .get_mut(&added_vertex)
            .unwrap()
            .push((end2, 0));
        self.adj_list
            .get_mut(&end2)
            .unwrap()
            .push((added_vertex, 0));

        // find minimum hamiltonian cycle
        let (min_cost, mut min_path) = self.hamiltonian_cycle_min();

        // since added_vertex edges are 0, min_cost is correct
        // min_path is actually min_cycle that needs to be transformed into min_path
        // added_vertex must be removed from it, and adjacent vertices
        // (guaranteed to be end1, end2) must be first and last vertices in the path

        // find the current position of the added_vertex
        let added_vertex_pos = min_path
            .iter()
            .position(|vertex| vertex == &added_vertex)
            .unwrap();

        // rotate min_path so that the added_vertex is at index 0
        min_path.rotate_left(added_vertex_pos);

        // remove added_vertex
        min_path.remove(0);

        // remove added_vertex from graph
        self.remove_vertex(added_vertex);

        // return min_cost and min_path
        (min_cost, min_path)
    }

    /// Finds the longest Hamiltonian path in the graph with fixed ends.
    /// Note that the ends are fixed, but not the direction of the path.
    /// Returns a tuple containing the maximum cost and the vertices in the path.
    pub fn hamiltonian_path_max_fixed_ends(
        &self,
        end1: &Vertex<T>,
        end2: &Vertex<T>,
    ) -> (U, Vec<&Vertex<T>>) {
        if self.adj_list.len() < 2 {
            panic!("The graph must contain at least 2 vertices.");
        }
        if !self.adj_list.contains_key(&end1) {
            panic!("end1 not present in the graph.");
        }
        if !self.adj_list.contains_key(&end2) {
            panic!("end2 not present in the graph.");
        }

        // add new vertex
        let added_vertex = self.new_vertex();

        // set edges between start and added_vertex, added_vertex and end, to 0
        self.adj_list
            .get_mut(&added_vertex)
            .unwrap()
            .push((end1, 0));
        self.adj_list
            .get_mut(&end1)
            .unwrap()
            .push((added_vertex, 0));
        self.adj_list
            .get_mut(&added_vertex)
            .unwrap()
            .push((end2, 0));
        self.adj_list
            .get_mut(&end2)
            .unwrap()
            .push((added_vertex, 0));

        // find maximum hamiltonian cycle
        let (max_cost, mut max_path) = self.hamiltonian_cycle_max();

        // since added_vertex edges are 0, max_cost is correct
        // max_path is actually max_cycle that needs to be transformed into max_path
        // added_vertex must be removed from it, and adjacent vertices
        // (guaranteed to be end1, end2) must be first and last vertices in the path

        // find the current position of the added_vertex
        let added_vertex_pos = max_path
            .iter()
            .position(|vertex| vertex == &added_vertex)
            .unwrap();

        // rotate max_path so that the added_vertex is at index 0
        max_path.rotate_left(added_vertex_pos);

        // remove added_vertex
        max_path.remove(0);

        // remove added_vertex from the graph
        self.remove_vertex(added_vertex);

        // return max_cost and max_path
        (max_cost, max_path)
    }
}
impl<T, U> Default for Graph<T, U>
where
    T: Eq + Hash + Clone,
    U: ConstZero + Copy + PartialOrd + Bounded,
{
    fn default() -> Self {
        Self::new()
    }
}

/// A struct representing a vertex in a graph.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Vertex<T> {
    value: T,
}
impl<T> Vertex<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
    pub fn data(&self) -> &T {
        &self.value
    }
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
impl<T> From<T> for Vertex<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
impl<T> Default for Vertex<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}
