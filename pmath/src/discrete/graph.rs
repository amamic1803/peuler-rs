//! Graphs and graph algorithms.

use either::Either;
use num_traits::{Bounded, ConstZero, Num};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::iter;
use std::rc::Rc;

/// A directed graph with weighted, non-negative edges.
/// # Example
/// ```
/// use pmath::discrete::graph::{Graph, Vertex};
///
/// let mut graph = Graph::new();
/// let v1 = Vertex::new("A");
/// let v2 = Vertex::new("B");
/// graph.add(&v1);
/// graph.add(&v2);
/// graph.set_edge(&v1, &v2, 5);
///
/// assert_eq!(graph.len(), 2);
/// assert_eq!(graph.edge(&v1, &v2), Some(5));
/// assert_eq!(graph.edge(&v2, &v1), None);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Graph<T, U>
where
    T: Eq + Hash,
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
    U: ConstZero + Copy + PartialOrd + Bounded + Num,
{
    /// Creates a new, empty [Graph].
    /// # Returns
    /// * A new, empty [Graph] instance.
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates a new [Graph] with a specified initial capacity (for vertices).
    /// # Arguments
    /// * `capacity` - The initial capacity for the vertices.
    /// # Returns
    /// * A new [Graph] instance with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            id_counter: 0,
            freed_ids: Vec::with_capacity(capacity),
            vertex2id: HashMap::with_capacity(capacity),
            id2vertex: HashMap::with_capacity(capacity),
            adj_list: HashMap::with_capacity(capacity),
        }
    }

    /// Sets the weight of the edge between two vertices.
    ///
    /// This creates the edge if it does not exist, updates the weight if it does exist, or removes the edge if `value` is `None`.
    /// # Arguments
    /// * `vertex1` - The first vertex.
    /// * `vertex2` - The second vertex.
    /// * `value` - The weight of the edge, or `None` to remove the edge.
    /// # Panics
    /// * If either `vertex1` or `vertex2` is not present in the graph.
    /// * If `value` is negative.
    pub fn set_edge(&mut self, vertex1: &Vertex<T>, vertex2: &Vertex<T>, value: Option<U>) {
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

        match value {
            Some(v) => {
                if v < U::ZERO {
                    panic!("Edge value must be non-negative.");
                }
                edges1.insert(id2, v);
            }
            None => {
                edges1.remove(&id2);
            }
        }
    }

    /// The weight of the edge between two vertices, if it exists.
    /// # Arguments
    /// * `vertex1` - The first vertex.
    /// * `vertex2` - The second vertex.
    /// # Returns
    /// * `Some(weight)` if the edge exists, where `weight` is the weight of the edge.
    /// * `None` if the edge does not exist.
    /// # Panics
    /// * If either `vertex1` or `vertex2` is not present in the graph.
    pub fn edge(&self, vertex1: &Vertex<T>, vertex2: &Vertex<T>) -> Option<U> {
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

        edges1.get(&id2).copied()
    }

    /// Adds a vertex to the graph.
    ///
    /// If the vertex already exists in the graph, this method does nothing.
    /// # Arguments
    /// * `vertex` - The vertex to add.
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

    /// Removes a vertex from the graph.
    ///
    /// If the vertex does not exist in the graph, this method does nothing.
    /// # Arguments
    /// * `vertex` - The vertex to remove.
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
    /// # Arguments
    /// * `vertex` - The vertex to check for.
    /// # Returns
    /// * `true` if the vertex exists in the graph, `false` otherwise.
    pub fn contains(&self, vertex: &Vertex<T>) -> bool {
        self.vertex2id.contains_key(vertex)
    }

    /// The number of vertices in the graph.
    /// # Returns
    /// * The number of vertices in the graph.
    pub fn len(&self) -> usize {
        self.vertex2id.len()
    }

    /// Checks if the graph is empty (contains no vertices).
    /// # Returns
    /// * `true` if the graph is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the iterator over the vertices in the graph.
    /// # Returns
    /// * An iterator over references to the vertices in the graph.
    pub fn vertices(&self) -> impl Iterator<Item = &Vertex<T>> {
        self.vertex2id.keys().map(|vertex| vertex.as_ref())
    }

    /// Returns the iterator over the edges in the graph.
    /// # Returns
    /// * An iterator over tuples of the form `(source vertex, target vertex, weight)`.
    pub fn edges(&self) -> impl Iterator<Item = (&Vertex<T>, &Vertex<T>, U)> {
        self.adj_list.iter().flat_map(move |(id1, edges)| {
            let vertex1 = self.id2vertex[id1].as_ref();
            edges.iter().map(move |(id2, &weight)| {
                let vertex2 = self.id2vertex[id2].as_ref();
                (vertex1, vertex2, weight)
            })
        })
    }

    /// Converts a vector of vertex IDs to a vector of references to the corresponding vertices.
    /// # Arguments
    /// * `ids` - A vector of vertex IDs.
    /// # Returns
    /// * A vector of references to the corresponding vertices.
    fn vec_id2vertex(&self, ids: Vec<usize>) -> Vec<&Vertex<T>> {
        ids.into_iter()
            .map(|id| self.id2vertex.get(&id).unwrap().as_ref())
            .collect()
    }

    /// Finds a minimum or maximum cost Hamiltonian cycle in the graph.
    /// # Arguments
    /// * `minimum` - If true, finds the minimum cost cycle; otherwise, finds the maximum cost cycle.
    /// * `weight` - A function that takes two vertex IDs and returns the weight of the edge between them, or `None` if no edge exists.
    /// * `neighbours` - A function that takes a vertex ID and returns an iterator over its neighbors and the weights of the edges to them.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, cycle)` if a Hamiltonian cycle is found, or `None` otherwise. The `cycle` is represented as a vector of vertex IDs.
    fn hamiltonian_core<X, Y, Z>(
        &self,
        minimum: bool,
        weight: X,
        neighbours: Y,
    ) -> Option<(U, Vec<usize>)>
    where
        X: Fn(&usize, &usize) -> Option<U>,
        Y: Fn(&usize) -> Z,
        Z: IntoIterator<Item = (usize, U)>,
    {
        struct Node<V> {
            value: usize, // vertex id
            cost: V,      // min/max possible cost for the full cycle following this node
            prev_node: Option<Rc<Node<V>>>,
            minimum: bool, // if searching for minimum or maximum cycle
        }
        impl<V> Node<V> {
            fn new(value: usize, cost: V, prev_node: Option<Rc<Node<V>>>, minimum: bool) -> Self {
                Self {
                    value,
                    cost,
                    prev_node,
                    minimum,
                }
            }
        }
        impl<V: PartialEq> PartialEq<Self> for Node<V> {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value && self.cost == other.cost
            }
        }
        impl<V: PartialEq> Eq for Node<V> {}
        impl<V: PartialOrd> PartialOrd for Node<V> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl<V: PartialOrd> Ord for Node<V> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.cost
                    .partial_cmp(&other.cost)
                    .map(|o| if self.minimum { o.reverse() } else { o })
                    .unwrap_or(Ordering::Equal)
            }
        }

        // initialize the best node and the best cost
        let best_cost_init = if minimum {
            U::max_value()
        } else {
            U::min_value()
        };
        let mut best_node = None;
        let mut best_cost = best_cost_init;

        // best edges optimization, best (minimum or maximum) edge weight of future nodes is
        // added to the cost of the current node, so that cost can be smaller/bigger than just
        // counting the cost from the start so the node can hopefully be pruned earlier
        // as it is impossible for the total cost to be better than the sum of
        // weights up to the current node and best edge weights of future nodes
        let mut best_edges = HashMap::with_capacity(self.len());
        for vertex in self.id2vertex.keys() {
            let mut best = best_cost_init;
            if minimum {
                for (_, weight) in neighbours(vertex).into_iter() {
                    if weight < best {
                        best = weight;
                    }
                }
            } else {
                for (_, weight) in neighbours(vertex).into_iter() {
                    if weight > best {
                        best = weight;
                    }
                }
            }
            if best == best_cost_init {
                return None; // no edges from this vertex, so no cycle is possible
            }
            best_edges.insert(*vertex, best);
        }

        // priority queue (max popped first)
        let mut queue = BinaryHeap::new();

        // take random vertex as starting point
        // it doesn't matter which vertex is chosen as the starting point
        // because the cycle can be rotated to start from any vertex
        let start_node = match self.id2vertex.keys().next() {
            Some(start_vertex) => {
                let mut best_edge_sum = U::ZERO;
                for weight in best_edges.values().copied() {
                    best_edge_sum = best_edge_sum + weight;
                }
                Node::new(*start_vertex, best_edge_sum, None, minimum)
            }
            None => return None, // no vertices in the graph
        };

        // add starting node to the queue
        queue.push(start_node);

        // process nodes until the queue is empty
        // or the cost for the popped node is less/greater than absolute best_cost
        // (all other nodes must have even worse cost than best_cost since this is
        // a priority queue, therefore there is no point in processing them)
        let mut visited_set = HashSet::with_capacity(self.len());
        while let Some(mut node) = queue.pop() {
            if minimum {
                if node.cost > best_cost {
                    break;
                }
            } else {
                if node.cost < best_cost {
                    break;
                }
            }

            visited_set.clear();
            let mut current_node = Some(&node);
            let mut first_node = &node;
            while let Some(node) = current_node {
                visited_set.insert(node.value);
                first_node = node;
                current_node = node.prev_node.as_ref().map(|prev_node| prev_node.as_ref());
            }

            if visited_set.len() == self.len() {
                // if all vertices have been visited, process final edge
                // and update best_cost and best_node if necessary

                match weight(&node.value, &first_node.value) {
                    Some(w) => node.cost = node.cost + w,
                    None => continue, // no edge, no cycle, discard this node
                }
                node.cost = node.cost - best_edges[&node.value];

                if minimum {
                    if node.cost < best_cost {
                        best_cost = node.cost;
                        best_node = Some(node);
                    }
                } else {
                    if node.cost > best_cost {
                        best_cost = node.cost;
                        best_node = Some(node);
                    }
                }
            } else {
                // if there are unvisited vertices, consider all unvisited neighbors of the current
                // vertex and add those to the queue with updated cost
                let node = Rc::new(node);
                for (neighbour, weight) in neighbours(&node.value).into_iter() {
                    if !visited_set.contains(&neighbour) {
                        let cost = node.cost - best_edges[&node.value] + weight;
                        let new_node = Node::new(neighbour, cost, Some(node.clone()), minimum);
                        // if the new node has the potential to give better result, add to queue,
                        // otherwise discard it
                        if minimum {
                            if new_node.cost < best_cost {
                                queue.push(new_node)
                            }
                        } else {
                            if new_node.cost > best_cost {
                                queue.push(new_node)
                            }
                        }
                    }
                }
            }
        }

        // if best_cost is still at its initial value, no cycle was found
        // else return the minimum cost and the cycle
        if best_cost == best_cost_init {
            None
        } else {
            let mut best_cycle = Vec::with_capacity(self.len());
            let mut current_node = best_node.as_ref();
            while let Some(node) = current_node {
                best_cycle.push(node.value);
                current_node = node.prev_node.as_ref().map(|prev_node| prev_node.as_ref());
            }
            best_cycle.reverse();
            Some((best_cost, best_cycle))
        }
    }

    /// Finds a minimum or maximum cost Hamiltonian cycle in the graph.
    /// # Arguments
    /// * `minimum` - If true, finds the minimum cost cycle; otherwise, finds the maximum cost cycle.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, cycle)` if a Hamiltonian cycle is found, or `None` otherwise.
    fn hamiltonian_cycle(&self, minimum: bool) -> Option<(U, Vec<&Vertex<T>>)> {
        let weight = |v1: &usize, v2: &usize| self.adj_list[v1].get(v2).copied();
        let neighbours = |v: &usize| self.adj_list[v].iter().map(|(&id, &weight)| (id, weight));
        self.hamiltonian_core(minimum, weight, neighbours)
            .map(|(cost, cycle)| (cost, self.vec_id2vertex(cycle)))
    }

    /// Finds a minimum or maximum cost Hamiltonian path in the graph.
    /// # Arguments
    /// * `minimum` - If true, finds the minimum cost path; otherwise, finds the maximum cost path.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, path)` if a Hamiltonian path is found, or `None` otherwise.
    fn hamiltonian_path(&self, minimum: bool) -> Option<(U, Vec<&Vertex<T>>)> {
        // add additional vertex to the graph with edges to all other vertices with weight 0
        // then, after the Hamiltonian cycle is found, remove the additional vertex and return the path
        // additional vertex will have id usize::MAX, which is guaranteed to be unique since the graph can have at most usize::MAX - 1 vertices
        let weight = |v1: &usize, v2: &usize| {
            if (*v1 == usize::MAX && *v2 != usize::MAX) || (*v1 != usize::MAX && *v2 == usize::MAX)
            {
                Some(U::ZERO)
            } else {
                self.adj_list[v1].get(v2).copied()
            }
        };
        let neighbours = |v: &usize| {
            if *v == usize::MAX {
                Either::Left(self.id2vertex.keys().map(|&id| (id, U::ZERO)))
            } else {
                Either::Right(
                    self.adj_list[v]
                        .iter()
                        .map(|(&id, &weight)| (id, weight))
                        .chain(iter::once((usize::MAX, U::ZERO))),
                )
            }
        };
        self.hamiltonian_core(minimum, weight, neighbours)
            .map(|(cost, cycle)| (cost, self.vec_id2vertex(cycle)))
    }

    /// Finds a minimum or maximum cost Hamiltonian path in the graph with fixed start and end vertices.
    /// # Arguments
    /// * `start` - The starting vertex of the path.
    /// * `end` - The ending vertex of the path.
    /// * `minimum` - If true, finds the minimum cost path; otherwise, finds the maximum cost path.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, path)` if a Hamiltonian path is found, or `None` otherwise.
    /// # Panics
    /// * If either `start` or `end` is not present in the graph.
    fn hamiltonian_path_fixed_ends(
        &self,
        start: &Vertex<T>,
        end: &Vertex<T>,
        minimum: bool,
    ) -> Option<(U, Vec<&Vertex<T>>)> {
        // add additional vertex to the graph with 2 edges,
        // one from this vertex to start, and another from end to this vertex
        // both with weight 0
        // after finding the cycle and removing this additional vertex, the result is a Hamiltonian
        // path which starts at start and ends at end
        // the total cost is not affected since the additional edges have weight 0
        // additional vertex will have id usize::MAX,
        // which is guaranteed to be unique since the graph can have at most usize::MAX - 1 vertices
        let start_id = match self.vertex2id.get(start) {
            Some(id) => *id,
            None => panic!("start vertex not present in the graph."),
        };
        let end_id = match self.vertex2id.get(end) {
            Some(id) => *id,
            None => panic!("end vertex not present in the graph."),
        };
        let weight = |v1: &usize, v2: &usize| {
            if (*v1 == usize::MAX && *v2 == start_id) || (*v1 == end_id && *v2 == usize::MAX) {
                Some(U::ZERO)
            } else {
                self.adj_list[v1].get(v2).copied()
            }
        };
        let neighbours = |v: &usize| {
            if *v == usize::MAX {
                Either::Left(iter::once((start_id, U::ZERO)))
            } else if *v == end_id {
                Either::Left(iter::once((usize::MAX, U::ZERO)))
            } else {
                Either::Right(self.adj_list[v].iter().map(|(&id, &weight)| (id, weight)))
            }
        };
        self.hamiltonian_core(minimum, weight, neighbours)
            .map(|(cost, cycle)| (cost, self.vec_id2vertex(cycle)))
    }

    /// Finds the minimum cost Hamiltonian cycle in the graph.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, cycle)` if a Hamiltonian cycle is found, or `None` otherwise.
    pub fn hamiltonian_cycle_min(&self) -> Option<(U, Vec<&Vertex<T>>)> {
        self.hamiltonian_cycle(true)
    }

    /// Finds the maximum cost Hamiltonian cycle in the graph.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, cycle)` if a Hamiltonian cycle is found, or `None` otherwise.
    pub fn hamiltonian_cycle_max(&self) -> Option<(U, Vec<&Vertex<T>>)> {
        self.hamiltonian_cycle(false)
    }

    /// Finds the minimum cost Hamiltonian path in the graph.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, path)` if a Hamiltonian path is found, or `None` otherwise.
    pub fn hamiltonian_path_min(&self) -> Option<(U, Vec<&Vertex<T>>)> {
        self.hamiltonian_path(true)
    }

    /// Finds the maximum cost Hamiltonian path in the graph.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, path)` if a Hamiltonian path is found, or `None` otherwise.
    pub fn hamiltonian_path_max(&self) -> Option<(U, Vec<&Vertex<T>>)> {
        self.hamiltonian_path(false)
    }

    /// Finds the minimum cost Hamiltonian path in the graph with fixed end vertices.
    /// # Arguments
    /// * `start` - The starting vertex of the path.
    /// * `end` - The ending vertex of the path.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, path)` if a Hamiltonian path is found, or `None` otherwise.
    /// # Panics
    /// * If either `start` or `end` is not present in the graph.
    pub fn hamiltonian_path_fixed_ends_min(
        &self,
        start: &Vertex<T>,
        end: &Vertex<T>,
    ) -> Option<(U, Vec<&Vertex<T>>)> {
        self.hamiltonian_path_fixed_ends(start, end, true)
    }

    /// Finds the maximum cost Hamiltonian path in the graph with fixed end vertices.
    /// # Arguments
    /// * `start` - The starting vertex of the path.
    /// * `end` - The ending vertex of the path.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, path)` if a Hamiltonian path is found, or `None` otherwise.
    /// # Panics
    /// * If either `start` or `end` is not present in the graph.
    pub fn hamiltonian_path_max_fixed_ends(
        &self,
        start: &Vertex<T>,
        end: &Vertex<T>,
    ) -> Option<(U, Vec<&Vertex<T>>)> {
        self.hamiltonian_path_fixed_ends(start, end, false)
    }
}
impl<T, U> Default for Graph<T, U>
where
    T: Eq + Hash + Clone,
    U: ConstZero + Copy + PartialOrd + Bounded + Num,
{
    fn default() -> Self {
        Self::new()
    }
}

/// A vertex in a [Graph].
/// # Example
/// ```
/// use pmath::discrete::graph::Vertex;
///
/// let vertex = Vertex::new("A");
/// assert_eq!(vertex.value(), &"A");
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Vertex<T> {
    value: T,
}
impl<T> Vertex<T> {
    /// Creates a new [Vertex] with the given value.
    /// # Arguments
    /// * `value` - The value associated with the vertex.
    /// # Returns
    /// * A new [Vertex] instance containing the given value.
    pub fn new(value: T) -> Self {
        Self { value }
    }
    /// Returns a reference to the value associated with the vertex.
    /// # Returns
    /// * A reference to the value of the vertex.
    pub fn value(&self) -> &T {
        &self.value
    }
    /// Returns a mutable reference to the value associated with the vertex.
    /// # Returns
    /// * A mutable reference to the value of the vertex.
    pub fn value_mut(&mut self) -> &mut T {
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

#[cfg(test)]
mod tests {}
