//! Graphs and graph algorithms.

use either::Either;
use num_traits::{Bounded, ConstZero, Num};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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
/// graph.set_edge(&v1, &v2, Some(5));
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

    /// Sets the weight of the edge between two vertices in both directions.
    ///
    /// This creates the edge if it does not exist, updates the weight if it does exist, or removes the edge if `value` is `None`.
    /// # Arguments
    /// * `vertex1` - The first vertex.
    /// * `vertex2` - The second vertex.
    /// * `value` - The weight of the edges, or `None` to remove the edges.
    /// # Panics
    /// * If either `vertex1` or `vertex2` is not present in the graph.
    /// * If `value` is negative.
    pub fn set_edge_bidirectional(
        &mut self,
        vertex1: &Vertex<T>,
        vertex2: &Vertex<T>,
        value: Option<U>,
    ) {
        self.set_edge(vertex1, vertex2, value);
        self.set_edge(vertex2, vertex1, value);
    }

    /// Returns the iterator over the neighbors of a vertex in the graph.
    /// # Arguments
    /// * `vertex` - The vertex to get the neighbors of.
    /// # Returns
    /// * An iterator over tuples of the form `(neighbor, weight)`.
    /// # Panics
    /// * If the vertex is not present in the graph.
    pub fn neighbors(&self, vertex: &Vertex<T>) -> impl Iterator<Item = (&Vertex<T>, U)> {
        match self.vertex2id.get(vertex) {
            Some(id) => self.adj_list[id].iter().map(|(id2, &weight)| {
                let vertex2 = self.id2vertex[id2].as_ref();
                (vertex2, weight)
            }),
            None => panic!("vertex not present in the graph."),
        }
    }

    /// Transforms the result of the hamiltonian core methods.
    ///
    /// This involves rotating the cycle to start with the first vertex,
    /// removing additional vertex used for path finding,
    /// and mapping the vertex ids back to the actual vertices.
    fn hamiltonian_transform_result(&self, mut cycle: Vec<usize>) -> Vec<&Vertex<T>> {
        if let Some(vertex_location) = cycle.iter().position(|&id| id == usize::MAX) {
            cycle.rotate_left(vertex_location + 1);
            cycle.pop();
        }
        cycle
            .into_iter()
            .map(|id| self.id2vertex[&id].as_ref())
            .collect()
    }

    /// Finds a Hamiltonian cycle in the graph.
    /// # Arguments
    /// * `vertices` - An iterator over the vertex IDs in the graph.
    /// * `edge` - A function that takes two vertex IDs and returns `true` if there is an edge between them, or `false` otherwise.
    /// * `neighbors` - A function that takes a vertex ID and returns an iterator over its neighbors.
    /// # Returns
    /// * An [Option] containing a vector of vertex IDs representing the Hamiltonian cycle if one is found, or `None` otherwise.
    fn hamiltonian_core<X, Y, Z, Q>(vertices: Q, edge: X, neighbors: Y) -> Option<Vec<usize>>
    where
        X: Fn(&usize, &usize) -> bool,
        Y: Fn(&usize) -> Z,
        Z: IntoIterator<Item = usize>,
        Q: IntoIterator<Item = usize>,
    {
        struct Node {
            value: usize, // vertex id
            prev_node: Option<Rc<Node>>,
        }
        impl Node {
            fn new(value: usize, prev_node: Option<Rc<Node>>) -> Self {
                Self { value, prev_node }
            }
        }

        let mut vertices = vertices.into_iter().peekable();

        let mut queue = VecDeque::new();
        let start_node = Node::new(*vertices.peek()?, None);
        queue.push_back(start_node);

        let len = vertices.count();

        let mut visited_set = HashSet::with_capacity(len);
        while let Some(node) = queue.pop_front() {
            visited_set.clear();
            let mut current_node = Some(&node);
            let mut first_node = &node;
            while let Some(node) = current_node {
                visited_set.insert(node.value);
                first_node = node;
                current_node = node.prev_node.as_ref().map(|prev_node| prev_node.as_ref());
            }

            if visited_set.len() == len {
                // if all vertices have been visited, check final edge
                // if it exists, return the cycle
                if edge(&node.value, &first_node.value) {
                    let mut cycle = Vec::new();
                    let mut current_node = Some(&node);
                    while let Some(n) = current_node {
                        cycle.push(n.value);
                        current_node = n.prev_node.as_ref().map(|prev_node| prev_node.as_ref());
                    }
                    cycle.reverse();
                    return Some(cycle);
                }
            } else {
                // if there are unvisited vertices, consider all unvisited neighbors of the current
                // vertex and add those to the queue
                let node = Rc::new(node);
                for neighbour in neighbors(&node.value).into_iter() {
                    if !visited_set.contains(&neighbour) {
                        queue.push_back(Node::new(neighbour, Some(node.clone())));
                    }
                }
            }
        }

        None
    }

    /// Finds a minimum or maximum cost Hamiltonian cycle in the graph.
    /// # Arguments
    /// * `vertices` - An iterator over the vertex IDs in the graph.
    /// * `weight` - A function that takes two vertex IDs and returns the weight of the edge between them, or `None` if no edge exists.
    /// * `neighbors` - A function that takes a vertex ID and returns an iterator over its neighbors and the weights of the edges to them.
    /// * `minimum` - If true, finds the minimum cost cycle; otherwise, finds the maximum cost cycle.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, cycle)` if a Hamiltonian cycle is found, or `None` otherwise. The `cycle` is represented as a vector of vertex IDs.
    fn hamiltonian_minmax_core<X, Y, Z, Q>(
        vertices: Q,
        weight: X,
        neighbors: Y,
        minimum: bool,
    ) -> Option<(U, Vec<usize>)>
    where
        X: Fn(&usize, &usize) -> Option<U>,
        Y: Fn(&usize) -> Z,
        Z: IntoIterator<Item = (usize, U)>,
        Q: IntoIterator<Item = usize>,
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

        let mut vertices = vertices.into_iter().peekable();

        // initialize the best node and the best cost
        let best_cost_init = if minimum {
            U::max_value()
        } else {
            U::min_value()
        };
        let mut best_node = None;
        let mut best_cost = best_cost_init;

        // take random vertex as starting point
        // it doesn't matter which vertex is chosen as the starting point
        // because the cycle can be rotated to start from any vertex
        let start_vertex = *vertices.peek()?;

        // the total number of vertices in the graph
        let mut len = 0;

        // best edges optimization, best (minimum or maximum) edge weight of future nodes is
        // added to the cost of the current node, so that cost can be smaller/bigger than just
        // counting the cost from the start so the node can hopefully be pruned earlier
        // as it is impossible for the total cost to be better than the sum of
        // weights up to the current node and best edge weights of future nodes
        let mut best_edges = HashMap::new();
        for vertex in vertices {
            len += 1;
            let mut best = best_cost_init;
            if minimum {
                for (_, weight) in neighbors(&vertex).into_iter() {
                    if weight < best {
                        best = weight;
                    }
                }
            } else {
                for (_, weight) in neighbors(&vertex).into_iter() {
                    if weight > best {
                        best = weight;
                    }
                }
            }
            if best == best_cost_init {
                return None; // no edges from this vertex, so no cycle is possible
            }
            best_edges.insert(vertex, best);
        }

        // priority queue (max popped first)
        let mut queue = BinaryHeap::new();

        // create starting node from starting vertex
        let start_node = {
            let mut best_edge_sum = U::ZERO;
            for weight in best_edges.values().copied() {
                best_edge_sum = best_edge_sum + weight;
            }
            Node::new(start_vertex, best_edge_sum, None, minimum)
        };

        // add starting node to the queue
        queue.push(start_node);

        // process nodes until the queue is empty
        // or the cost for the popped node is less/greater than absolute best_cost
        // (all other nodes must have even worse cost than best_cost since this is
        // a priority queue, therefore there is no point in processing them)
        let mut visited_set = HashSet::with_capacity(len);
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

            if visited_set.len() == len {
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
                for (neighbour, weight) in neighbors(&node.value).into_iter() {
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
            let mut best_cycle = Vec::with_capacity(len);
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
    fn hamiltonian_cycle_minmax(&self, minimum: bool) -> Option<(U, Vec<&Vertex<T>>)> {
        let vertices = self.id2vertex.keys().copied();
        let weight = |v1: &usize, v2: &usize| self.adj_list[v1].get(v2).copied();
        let neighbors = |v: &usize| self.adj_list[v].iter().map(|(&id, &weight)| (id, weight));
        Self::hamiltonian_minmax_core(vertices, weight, neighbors, minimum)
            .map(|(cost, cycle)| (cost, self.hamiltonian_transform_result(cycle)))
    }

    /// Finds a minimum or maximum cost Hamiltonian path in the graph.
    /// # Arguments
    /// * `minimum` - If true, finds the minimum cost path; otherwise, finds the maximum cost path.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, path)` if a Hamiltonian path is found, or `None` otherwise.
    fn hamiltonian_path_minmax(&self, minimum: bool) -> Option<(U, Vec<&Vertex<T>>)> {
        // add additional vertex to the graph with edges to all other vertices with weight 0
        // then, after the Hamiltonian cycle is found, remove the additional vertex and return the path
        // additional vertex will have id usize::MAX, which is guaranteed to be unique since the graph can have at most usize::MAX - 1 vertices
        let vertices = self.id2vertex.keys().copied().chain(iter::once(usize::MAX));
        let weight = |v1: &usize, v2: &usize| {
            if *v1 == usize::MAX {
                if *v2 == usize::MAX {
                    None
                } else {
                    Some(U::ZERO)
                }
            } else {
                if *v2 == usize::MAX {
                    Some(U::ZERO)
                } else {
                    self.adj_list[v1].get(v2).copied()
                }
            }
        };
        let neighbors = |v: &usize| {
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
        Self::hamiltonian_minmax_core(vertices, weight, neighbors, minimum)
            .map(|(cost, cycle)| (cost, self.hamiltonian_transform_result(cycle)))
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
    fn hamiltonian_path_fixed_ends_minmax(
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
        let vertices = self.id2vertex.keys().copied().chain(iter::once(usize::MAX));
        let weight = |v1: &usize, v2: &usize| {
            if *v1 == usize::MAX {
                if *v2 == start_id { Some(U::ZERO) } else { None }
            } else if *v2 == usize::MAX {
                if *v1 == end_id { Some(U::ZERO) } else { None }
            } else {
                self.adj_list[v1].get(v2).copied()
            }
        };
        let neighbors = |v: &usize| {
            if *v == usize::MAX {
                Either::Left(Either::Left(iter::once((start_id, U::ZERO))))
            } else if *v == end_id {
                Either::Left(Either::Right(
                    self.adj_list[v]
                        .iter()
                        .map(|(&id, &weight)| (id, weight))
                        .chain(iter::once((usize::MAX, U::ZERO))),
                ))
            } else {
                Either::Right(self.adj_list[v].iter().map(|(&id, &weight)| (id, weight)))
            }
        };
        Self::hamiltonian_minmax_core(vertices, weight, neighbors, minimum)
            .map(|(cost, cycle)| (cost, self.hamiltonian_transform_result(cycle)))
    }

    /// Finds a Hamiltonian cycle in the graph.
    /// # Returns
    /// * An [Option] containing a vector of references to the vertices in the Hamiltonian cycle if one is found, or `None` otherwise.
    pub fn hamiltonian_cycle(&self) -> Option<Vec<&Vertex<T>>> {
        let vertices = self.id2vertex.keys().copied();
        let edge = |v1: &usize, v2: &usize| self.adj_list[v1].contains_key(v2);
        let neighbors = |v: &usize| self.adj_list[v].keys().copied();
        Self::hamiltonian_core(vertices, edge, neighbors)
            .map(|cycle| self.hamiltonian_transform_result(cycle))
    }

    /// Finds the minimum cost Hamiltonian cycle in the graph.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, cycle)` if a Hamiltonian cycle is found, or `None` otherwise.
    pub fn hamiltonian_cycle_min(&self) -> Option<(U, Vec<&Vertex<T>>)> {
        self.hamiltonian_cycle_minmax(true)
    }

    /// Finds the maximum cost Hamiltonian cycle in the graph.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, cycle)` if a Hamiltonian cycle is found, or `None` otherwise.
    pub fn hamiltonian_cycle_max(&self) -> Option<(U, Vec<&Vertex<T>>)> {
        self.hamiltonian_cycle_minmax(false)
    }

    /// Finds a Hamiltonian path in the graph.
    /// # Returns
    /// * An [Option] containing a vector of references to the vertices in the Hamiltonian path if one is found, or `None` otherwise.
    pub fn hamiltonian_path(&self) -> Option<Vec<&Vertex<T>>> {
        // add additional vertex to the graph with edges to all other vertices
        // then, after the Hamiltonian cycle is found, remove the additional vertex and return the path
        // additional vertex will have id usize::MAX, which is guaranteed to be unique since the graph can have at most usize::MAX - 1 vertices
        let vertices = self.id2vertex.keys().copied().chain(iter::once(usize::MAX));
        let edge = |v1: &usize, v2: &usize| {
            if *v1 == usize::MAX {
                *v2 != usize::MAX
            } else if *v2 == usize::MAX {
                *v1 != usize::MAX
            } else {
                self.adj_list[v1].contains_key(v2)
            }
        };
        let neighbors = |v: &usize| {
            if *v == usize::MAX {
                Either::Left(self.id2vertex.keys().copied())
            } else {
                Either::Right(
                    self.adj_list[v]
                        .keys()
                        .copied()
                        .chain(iter::once(usize::MAX)),
                )
            }
        };
        Self::hamiltonian_core(vertices, edge, neighbors)
            .map(|cycle| self.hamiltonian_transform_result(cycle))
    }

    /// Finds the minimum cost Hamiltonian path in the graph.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, path)` if a Hamiltonian path is found, or `None` otherwise.
    pub fn hamiltonian_path_min(&self) -> Option<(U, Vec<&Vertex<T>>)> {
        self.hamiltonian_path_minmax(true)
    }

    /// Finds the maximum cost Hamiltonian path in the graph.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, path)` if a Hamiltonian path is found, or `None` otherwise.
    pub fn hamiltonian_path_max(&self) -> Option<(U, Vec<&Vertex<T>>)> {
        self.hamiltonian_path_minmax(false)
    }

    /// Finds a Hamiltonian path in the graph with fixed end vertices.
    /// # Arguments
    /// * `start` - The starting vertex of the path.
    /// * `end` - The ending vertex of the path.
    /// # Returns
    /// * An [Option] containing a vector of references to the vertices in the Hamiltonian path if one is found, or `None` otherwise.
    /// # Panics
    /// * If either `start` or `end` is not present in the graph.
    pub fn hamiltonian_path_fixed_ends(
        &self,
        start: &Vertex<T>,
        end: &Vertex<T>,
    ) -> Option<Vec<&Vertex<T>>> {
        // add additional vertex to the graph with 2 edges,
        // one from this vertex to start, and another from end to this vertex
        // after finding the cycle and removing this additional vertex, the result is a Hamiltonian
        // path which starts at start and ends at end
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
        let vertices = self.id2vertex.keys().copied().chain(iter::once(usize::MAX));
        let edge = |v1: &usize, v2: &usize| {
            if *v1 == usize::MAX {
                *v2 == start_id
            } else if *v2 == usize::MAX {
                *v1 == end_id
            } else {
                self.adj_list[v1].contains_key(v2)
            }
        };
        let neighbors = |v: &usize| {
            if *v == usize::MAX {
                Either::Left(Either::Left(iter::once(start_id)))
            } else if *v == end_id {
                Either::Left(Either::Right(
                    self.adj_list[v]
                        .keys()
                        .copied()
                        .chain(iter::once(usize::MAX)),
                ))
            } else {
                Either::Right(self.adj_list[v].keys().copied())
            }
        };
        Self::hamiltonian_core(vertices, edge, neighbors)
            .map(|cycle| self.hamiltonian_transform_result(cycle))
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
        self.hamiltonian_path_fixed_ends_minmax(start, end, true)
    }

    /// Finds the maximum cost Hamiltonian path in the graph with fixed end vertices.
    /// # Arguments
    /// * `start` - The starting vertex of the path.
    /// * `end` - The ending vertex of the path.
    /// # Returns
    /// * An [Option] containing a tuple of the form `(cost, path)` if a Hamiltonian path is found, or `None` otherwise.
    /// # Panics
    /// * If either `start` or `end` is not present in the graph.
    pub fn hamiltonian_path_fixed_ends_max(
        &self,
        start: &Vertex<T>,
        end: &Vertex<T>,
    ) -> Option<(U, Vec<&Vertex<T>>)> {
        self.hamiltonian_path_fixed_ends_minmax(start, end, false)
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
    pub const fn new(value: T) -> Self {
        Self { value }
    }
    /// Returns a reference to the value associated with the vertex.
    /// # Returns
    /// * A reference to the value of the vertex.
    pub const fn value(&self) -> &T {
        &self.value
    }
    /// Returns a mutable reference to the value associated with the vertex.
    /// # Returns
    /// * A mutable reference to the value of the vertex.
    pub const fn value_mut(&mut self) -> &mut T {
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
mod tests {
    use super::*;
    use std::panic::catch_unwind;

    // vertex tests

    #[test]
    fn vertex_new() {
        //! Test that [Vertex::new] creates a vertex with the correct value.

        let vertex = Vertex::new("A");
        assert_eq!(vertex.value(), &"A");
    }

    #[test]
    fn vertex_value() {
        //! Test that [Vertex::value] returns the correct value.

        let vertex = Vertex::new("A");
        assert_eq!(vertex.value(), &"A");
    }

    #[test]
    fn vertex_value_mut() {
        //! Test that [Vertex::value_mut] returns a mutable reference to the correct value.

        let mut vertex = Vertex::new("A");
        *vertex.value_mut() = "B";
        assert_eq!(vertex.value(), &"B");
    }

    #[test]
    fn vertex_from() {
        //! Test [From] implementation for [Vertex].

        let vertex = Vertex::from("A");
        assert_eq!(vertex.value(), &"A");

        let value = 42;
        let vertex_int: Vertex<_> = value.into();
        assert_eq!(vertex_int.value(), &42);
    }

    #[test]
    fn vertex_default() {
        //! Test that [Vertex::default] creates a vertex with the default value.

        let vertex = Vertex::<String>::default();
        assert_eq!(vertex.value(), &String::default());
    }

    #[test]
    fn vertex_generics() {
        //! Test that [Vertex] can be created with different types of values.

        let vertex_int = Vertex::new(42);
        assert_eq!(vertex_int.value(), &42);

        let vertex_string = Vertex::new(String::from("Hello"));
        assert_eq!(vertex_string.value(), &String::from("Hello"));

        let vertex_float = Vertex::new(3.5);
        assert_eq!(vertex_float.value(), &3.5);

        let vertex_tuple = Vertex::new((1, "A"));
        assert_eq!(vertex_tuple.value(), &(1, "A"));

        let vertex_vec = Vertex::new(vec![1, 2, 3]);
        assert_eq!(vertex_vec.value(), &vec![1, 2, 3]);
    }

    // graph tests

    const TEST_GRAPH_SIZE: usize = 10;
    const TEST_GRAPH_VERTICES: [Vertex<&str>; TEST_GRAPH_SIZE] = [
        Vertex::new("A"),
        Vertex::new("B"),
        Vertex::new("C"),
        Vertex::new("D"),
        Vertex::new("E"),
        Vertex::new("F"),
        Vertex::new("G"),
        Vertex::new("H"),
        Vertex::new("I"),
        Vertex::new("J"),
    ];
    const TEST_GRAPH_ADJACENCY_MATRIX: [[Option<i32>; TEST_GRAPH_SIZE]; TEST_GRAPH_SIZE] = [
        [
            None,
            None,
            Some(2),
            Some(1),
            None,
            None,
            Some(5),
            Some(4),
            None,
            None,
        ],
        [
            None,
            None,
            Some(4),
            None,
            Some(9),
            Some(3),
            None,
            Some(2),
            None,
            None,
        ],
        [
            Some(1),
            None,
            None,
            Some(2),
            Some(7),
            None,
            Some(1),
            None,
            None,
            None,
        ],
        [
            None,
            Some(4),
            None,
            None,
            Some(9),
            Some(2),
            Some(4),
            None,
            None,
            None,
        ],
        [
            None,
            Some(4),
            None,
            Some(9),
            None,
            Some(1),
            None,
            Some(9),
            None,
            None,
        ],
        [
            Some(5),
            None,
            None,
            None,
            None,
            None,
            Some(7),
            None,
            Some(4),
            Some(8),
        ],
        [
            None,
            None,
            None,
            None,
            None,
            Some(6),
            None,
            Some(1),
            Some(3),
            Some(7),
        ],
        [
            Some(4),
            Some(6),
            None,
            None,
            None,
            None,
            None,
            None,
            Some(5),
            Some(3),
        ],
        [
            Some(2),
            None,
            Some(7),
            Some(2),
            None,
            None,
            None,
            None,
            None,
            Some(2),
        ],
        [
            None,
            Some(6),
            Some(6),
            None,
            Some(1),
            None,
            None,
            None,
            Some(5),
            None,
        ],
    ];
    const TEST_GRAPH_HAM_CYCLE_MIN_COST: i32 = 22;
    const TEST_GRAPH_HAM_CYCLE_MAX_COST: i32 = 59;
    const TEST_GRAPH_HAM_PATH_MIN_COST: i32 = 18;
    const TEST_GRAPH_HAM_PATH_MAX_COST: i32 = 58;
    const TEST_GRAPH_HAM_PATH_FIXED_ENDS_MIN_COST: i32 = 21; // A to D
    const TEST_GRAPH_HAM_PATH_FIXED_ENDS_MAX_COST: i32 = 57; // A to D

    fn init_test_graph() -> Graph<&'static str, i32> {
        let mut graph = Graph::<&str, i32>::new();
        for vertex in TEST_GRAPH_VERTICES.iter() {
            graph.add(vertex);
        }
        for (i, vertex) in TEST_GRAPH_VERTICES.iter().enumerate() {
            for (j, weight) in TEST_GRAPH_ADJACENCY_MATRIX[i].iter().enumerate() {
                if let Some(w) = weight {
                    graph.set_edge(vertex, &TEST_GRAPH_VERTICES[j], Some(*w));
                }
            }
        }
        graph
    }

    #[test]
    fn graph_new() {
        //! Test that [Graph::new] creates an empty graph.

        let graph = Graph::<String, i32>::new();
        assert_eq!(graph.len(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn graph_with_capacity() {
        //! Test that [Graph::with_capacity] creates an empty graph with the specified capacity.

        let capacity = 10;
        let graph = Graph::<String, i32>::with_capacity(capacity);
        assert_eq!(graph.len(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn graph_default() {
        //! Test that [Graph::default] creates an empty graph.

        let graph = Graph::<String, i32>::default();
        assert_eq!(graph.len(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn graph_generics() {
        //! Test that [Graph] can be created with different types of vertex values and edge weights.

        let graph_int = Graph::<i32, i32>::new();
        assert_eq!(graph_int.len(), 0);
        assert!(graph_int.is_empty());

        let graph_string = Graph::<String, i32>::new();
        assert_eq!(graph_string.len(), 0);
        assert!(graph_string.is_empty());

        let graph_str = Graph::<&str, i32>::new();
        assert_eq!(graph_str.len(), 0);
        assert!(graph_str.is_empty());

        let graph_tuple = Graph::<(i32, String), i32>::new();
        assert_eq!(graph_tuple.len(), 0);
        assert!(graph_tuple.is_empty());

        let graph_vec = Graph::<Vec<i32>, i32>::new();
        assert_eq!(graph_vec.len(), 0);
        assert!(graph_vec.is_empty());
    }

    #[test]
    fn graph_add() {
        //! Test that [Graph::add] adds a vertex to the graph.

        let mut graph = Graph::<&str, i32>::new();
        for vertex in TEST_GRAPH_VERTICES.iter() {
            graph.add(vertex);
        }
        assert_eq!(graph.len(), TEST_GRAPH_SIZE);
        assert!(!graph.is_empty());
    }

    #[test]
    fn graph_add_duplicate() {
        //! Test that adding a duplicate vertex does not increase the size of the graph.

        let mut graph = Graph::<_, i32>::new();
        for vertex in TEST_GRAPH_VERTICES.iter() {
            graph.add(vertex);
        }
        assert_eq!(graph.len(), TEST_GRAPH_SIZE);
        assert!(!graph.is_empty());

        // Add duplicate vertices
        for vertex in TEST_GRAPH_VERTICES.iter() {
            graph.add(vertex);
        }
        assert_eq!(graph.len(), TEST_GRAPH_SIZE);
    }

    #[test]
    fn graph_remove() {
        //! Test that [Graph::remove] removes a vertex from the graph.

        let mut graph = init_test_graph();
        assert_eq!(graph.len(), TEST_GRAPH_SIZE);
        assert!(!graph.is_empty());

        // Remove a vertex
        let vertex_to_remove = TEST_GRAPH_VERTICES[0];
        graph.remove(&vertex_to_remove);
        assert_eq!(graph.len(), TEST_GRAPH_SIZE - 1);
        assert!(!graph.is_empty());
    }

    #[test]
    fn graph_remove_nonexistent() {
        //! Test that removing a non-existent vertex does not change the size of the graph.

        let mut graph = init_test_graph();
        assert_eq!(graph.len(), TEST_GRAPH_SIZE);
        assert!(!graph.is_empty());

        // Remove a non-existent vertex
        let nonexistent_vertex = Vertex::new("Z");
        graph.remove(&nonexistent_vertex);
        assert_eq!(graph.len(), TEST_GRAPH_SIZE);
    }

    #[test]
    fn graph_contains() {
        //! Test that [Graph::contains] correctly identifies whether a vertex is in the graph.

        let graph = init_test_graph();
        assert_eq!(graph.len(), TEST_GRAPH_SIZE);
        assert!(!graph.is_empty());

        // Check for existing vertices
        for vertex in TEST_GRAPH_VERTICES.iter() {
            assert!(graph.contains(vertex));
        }

        // Check for a non-existent vertex
        let nonexistent_vertex = Vertex::new("Z");
        assert!(!graph.contains(&nonexistent_vertex));
    }

    #[test]
    fn graph_len() {
        //! Test that [Graph::len] returns the correct number of vertices in the graph.

        let mut graph = Graph::<_, i32>::new();
        assert_eq!(graph.len(), 0);

        for (i, vertex) in TEST_GRAPH_VERTICES.iter().enumerate() {
            graph.add(vertex);
            assert_eq!(graph.len(), i + 1);
        }

        // Remove a vertex and check length
        let vertex_to_remove = TEST_GRAPH_VERTICES[0];
        graph.remove(&vertex_to_remove);
        assert_eq!(graph.len(), TEST_GRAPH_SIZE - 1);
    }

    #[test]
    fn graph_is_empty() {
        //! Test that [Graph::is_empty] correctly identifies whether the graph is empty.

        let mut graph = Graph::<_, i32>::new();
        assert!(graph.is_empty());

        for vertex in TEST_GRAPH_VERTICES.iter() {
            graph.add(vertex);
            assert!(!graph.is_empty());
        }

        // Remove all vertices and check if the graph is empty
        for (i, vertex) in TEST_GRAPH_VERTICES.iter().enumerate() {
            graph.remove(vertex);
            if i < TEST_GRAPH_SIZE - 1 {
                assert!(!graph.is_empty());
            }
        }
        assert!(graph.is_empty());
    }

    #[test]
    fn graph_vertices() {
        //! Test that [Graph::vertices] returns the correct set of vertices in the graph.

        let mut graph = init_test_graph();
        assert_eq!(graph.len(), TEST_GRAPH_SIZE);
        assert!(!graph.is_empty());

        let vertices: HashSet<_> = graph.vertices().collect();
        let expected_vertices: HashSet<_> = TEST_GRAPH_VERTICES.iter().collect();
        assert_eq!(vertices, expected_vertices);

        graph.remove(&TEST_GRAPH_VERTICES[0]);
        let vertices_after_removal: HashSet<_> = graph.vertices().collect();
        let expected_vertices_after_removal: HashSet<_> = TEST_GRAPH_VERTICES[1..].iter().collect();
        assert_eq!(vertices_after_removal, expected_vertices_after_removal);
    }

    #[test]
    fn graph_edge() {
        //! Test that [Graph::edge] returns the correct edge weight between two vertices.

        let graph = init_test_graph();
        assert_eq!(graph.len(), TEST_GRAPH_SIZE);
        assert!(!graph.is_empty());

        for (i, vertex1) in TEST_GRAPH_VERTICES.iter().enumerate() {
            for (j, vertex2) in TEST_GRAPH_VERTICES.iter().enumerate() {
                let expected_weight = TEST_GRAPH_ADJACENCY_MATRIX[i][j];
                let actual_weight = graph.edge(vertex1, vertex2);
                assert_eq!(actual_weight, expected_weight);
            }
        }
    }

    #[test]
    fn graph_edge_nonexistent() {
        //! Test that [Graph::edge] panics when querying the edge to or from a non-existent vertex.

        let graph = init_test_graph();
        let nonexistent_vertex = Vertex::new("Z");
        let result1 = catch_unwind(|| graph.edge(&nonexistent_vertex, &TEST_GRAPH_VERTICES[0]));
        assert!(result1.is_err());
        let result2 = catch_unwind(|| graph.edge(&TEST_GRAPH_VERTICES[0], &nonexistent_vertex));
        assert!(result2.is_err());
    }

    #[test]
    fn graph_edges() {
        //! Test that [Graph::edges] returns the correct set of edges in the graph.

        let graph = init_test_graph();
        assert_eq!(graph.len(), TEST_GRAPH_SIZE);
        assert!(!graph.is_empty());

        let edges: HashSet<_> = graph.edges().collect();
        let mut expected_edges = HashSet::new();
        for (i, vertex1) in TEST_GRAPH_VERTICES.iter().enumerate() {
            for (j, vertex2) in TEST_GRAPH_VERTICES.iter().enumerate() {
                if let Some(weight) = TEST_GRAPH_ADJACENCY_MATRIX[i][j] {
                    expected_edges.insert((vertex1, vertex2, weight));
                }
            }
        }
        assert_eq!(edges, expected_edges);
    }

    #[test]
    fn graph_set_edge() {
        //! Test that [Graph::set_edge] correctly sets the edge weight between two vertices.

        let mut graph = init_test_graph();
        assert_eq!(graph.len(), TEST_GRAPH_SIZE);
        assert!(!graph.is_empty());

        // Set a new edge weight
        let vertex1 = TEST_GRAPH_VERTICES[0];
        let vertex2 = TEST_GRAPH_VERTICES[1];
        let new_weight = Some(10);
        graph.set_edge(&vertex1, &vertex2, new_weight);

        // Check that the edge weight was updated
        let actual_weight = graph.edge(&vertex1, &vertex2);
        assert_eq!(actual_weight, new_weight);

        // Remove the edge by setting it to None
        graph.set_edge(&vertex1, &vertex2, None);
        let actual_weight_after_removal = graph.edge(&vertex1, &vertex2);
        assert_eq!(actual_weight_after_removal, None);
    }

    #[test]
    #[should_panic]
    fn graph_set_edge_nonexistent() {
        //! Test that [Graph::set_edge] panics when trying to set the edge to or from a non-existent vertex.

        let mut graph = init_test_graph();
        let nonexistent_vertex = Vertex::new("Z");
        graph.set_edge(&nonexistent_vertex, &TEST_GRAPH_VERTICES[0], Some(1));
    }

    #[test]
    #[should_panic]
    fn graph_set_edge_negative() {
        //! Test that [Graph::set_edge] panics when trying to set a negative edge weight.

        let mut graph = init_test_graph();
        let vertex1 = TEST_GRAPH_VERTICES[0];
        let vertex2 = TEST_GRAPH_VERTICES[1];
        graph.set_edge(&vertex1, &vertex2, Some(-1));
    }

    #[test]
    fn graph_set_edge_bidirectional() {
        //! Test that [Graph::set_edge] correctly sets the edge weight in both directions for an undirected graph.

        let mut graph = init_test_graph();
        assert_eq!(graph.len(), TEST_GRAPH_SIZE);
        assert!(!graph.is_empty());

        // Set a new edge weight
        let vertex1 = TEST_GRAPH_VERTICES[0];
        let vertex2 = TEST_GRAPH_VERTICES[1];
        let new_weight = Some(10);
        graph.set_edge_bidirectional(&vertex1, &vertex2, new_weight);

        // Check that the edge weight was updated in both directions
        let actual_weight_1_to_2 = graph.edge(&vertex1, &vertex2);
        let actual_weight_2_to_1 = graph.edge(&vertex2, &vertex1);
        assert_eq!(actual_weight_1_to_2, new_weight);
        assert_eq!(actual_weight_2_to_1, new_weight);

        // Remove the edge by setting it to None
        graph.set_edge_bidirectional(&vertex1, &vertex2, None);
        let actual_weight_after_removal_1_to_2 = graph.edge(&vertex1, &vertex2);
        let actual_weight_after_removal_2_to_1 = graph.edge(&vertex2, &vertex1);
        assert_eq!(actual_weight_after_removal_1_to_2, None);
        assert_eq!(actual_weight_after_removal_2_to_1, None);
    }

    #[test]
    #[should_panic]
    fn graph_set_edge_bidirectional_nonexistent() {
        //! Test that [Graph::set_edge] panics when trying to set the edge in both directions for a non-existent vertex in an undirected graph.

        let mut graph = init_test_graph();
        let nonexistent_vertex = Vertex::new("Z");
        graph.set_edge(&nonexistent_vertex, &TEST_GRAPH_VERTICES[0], Some(1));
    }

    #[test]
    #[should_panic]
    fn graph_set_edge_bidirectional_negative() {
        //! Test that [Graph::set_edge] panics when trying to set a negative edge weight in both directions for an undirected graph.

        let mut graph = init_test_graph();
        let vertex1 = TEST_GRAPH_VERTICES[0];
        let vertex2 = TEST_GRAPH_VERTICES[1];
        graph.set_edge(&vertex1, &vertex2, Some(-1));
    }

    #[test]
    fn graph_neighbors() {
        //! Test that [Graph::neighbors] returns the correct set of neighbors for a given vertex.

        let graph = init_test_graph();
        assert_eq!(graph.len(), TEST_GRAPH_SIZE);
        assert!(!graph.is_empty());

        for (i, vertex) in TEST_GRAPH_VERTICES.iter().enumerate() {
            let neighbors: HashSet<_> = graph.neighbors(vertex).collect();
            let expected_neighbors: HashSet<_> = TEST_GRAPH_ADJACENCY_MATRIX[i]
                .iter()
                .enumerate()
                .filter_map(|(j, weight)| weight.as_ref().map(|w| (&TEST_GRAPH_VERTICES[j], *w)))
                .collect();
            assert_eq!(neighbors, expected_neighbors);
        }
    }

    #[test]
    #[should_panic]
    fn graph_neighbors_nonexistent() {
        //! Test that [Graph::neighbors] panics when querying the neighbors of a non-existent vertex.

        let graph = init_test_graph();
        let nonexistent_vertex = Vertex::new("Z");
        let _n = graph.neighbors(&nonexistent_vertex);
    }

    #[test]
    fn graph_hamiltonian_cycle() {
        //! Test that [Graph::hamiltonian_cycle] finds a Hamiltonian cycle in the graph.

        let graph = init_test_graph();
        let cycle = graph.hamiltonian_cycle();
        assert!(cycle.is_some());
        let cycle = cycle.unwrap();
        assert_eq!(
            cycle.iter().copied().collect::<HashSet<_>>().len(),
            TEST_GRAPH_SIZE
        );
        for i in 0..cycle.len() {
            let v1 = cycle[i];
            let v2 = cycle[(i + 1) % cycle.len()];
            assert!(graph.edge(v1, v2).is_some());
        }
    }

    #[test]
    fn graph_hamiltonian_cycle_min() {
        //! Test that [Graph::hamiltonian_cycle_min] finds the minimum cost Hamiltonian cycle in the graph.

        let graph = init_test_graph();
        let result = graph.hamiltonian_cycle_min();
        assert!(result.is_some());

        let (cost, cycle) = result.unwrap();
        assert_eq!(cost, TEST_GRAPH_HAM_CYCLE_MIN_COST);
        assert_eq!(
            cycle.iter().copied().collect::<HashSet<_>>().len(),
            TEST_GRAPH_SIZE
        );

        // check that the cost is correct
        let mut calculated_cost = 0;
        for i in 0..cycle.len() {
            let v1 = cycle[i];
            let v2 = cycle[(i + 1) % cycle.len()];
            calculated_cost += graph.edge(v1, v2).unwrap();
        }
        assert_eq!(cost, calculated_cost);
    }

    #[test]
    fn graph_hamiltonian_cycle_max() {
        //! Test that [Graph::hamiltonian_cycle_max] finds the maximum cost Hamiltonian cycle in the graph.

        let graph = init_test_graph();
        let result = graph.hamiltonian_cycle_max();
        assert!(result.is_some());

        let (cost, cycle) = result.unwrap();
        assert_eq!(cost, TEST_GRAPH_HAM_CYCLE_MAX_COST);
        assert_eq!(
            cycle.iter().copied().collect::<HashSet<_>>().len(),
            TEST_GRAPH_SIZE
        );
        // check that the cost is correct
        let mut calculated_cost = 0;
        for i in 0..cycle.len() {
            let v1 = cycle[i];
            let v2 = cycle[(i + 1) % cycle.len()];
            calculated_cost += graph.edge(v1, v2).unwrap();
        }
        assert_eq!(cost, calculated_cost);
    }

    #[test]
    fn graph_hamiltonian_path() {
        //! Test that [Graph::hamiltonian_path] finds a Hamiltonian path in the graph.

        let graph = init_test_graph();
        let path = graph.hamiltonian_path();
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(
            path.iter().copied().collect::<HashSet<_>>().len(),
            TEST_GRAPH_SIZE
        );
        for i in 0..path.len() - 1 {
            let v1 = path[i];
            let v2 = path[i + 1];
            assert!(graph.edge(v1, v2).is_some());
        }
    }

    #[test]
    fn graph_hamiltonian_path_min() {
        //! Test that [Graph::hamiltonian_path_min] finds the minimum cost Hamiltonian path in the graph.

        let graph = init_test_graph();
        let result = graph.hamiltonian_path_min();
        assert!(result.is_some());

        let (cost, path) = result.unwrap();
        assert_eq!(cost, TEST_GRAPH_HAM_PATH_MIN_COST);
        assert_eq!(
            path.iter().copied().collect::<HashSet<_>>().len(),
            TEST_GRAPH_SIZE
        );

        // check that the cost is correct
        let mut calculated_cost = 0;
        for i in 0..path.len() - 1 {
            let v1 = path[i];
            let v2 = path[i + 1];
            calculated_cost += graph.edge(v1, v2).unwrap();
        }
        assert_eq!(cost, calculated_cost);
    }

    #[test]
    fn graph_hamiltonian_path_max() {
        //! Test that [Graph::hamiltonian_path_max] finds the maximum cost Hamiltonian path in the graph.

        let graph = init_test_graph();
        let result = graph.hamiltonian_path_max();
        assert!(result.is_some());

        let (cost, path) = result.unwrap();
        assert_eq!(cost, TEST_GRAPH_HAM_PATH_MAX_COST);
        assert_eq!(
            path.iter().copied().collect::<HashSet<_>>().len(),
            TEST_GRAPH_SIZE
        );

        // check that the cost is correct
        let mut calculated_cost = 0;
        for i in 0..path.len() - 1 {
            let v1 = path[i];
            let v2 = path[i + 1];
            calculated_cost += graph.edge(v1, v2).unwrap();
        }
        assert_eq!(cost, calculated_cost);
    }

    #[test]
    fn graph_hamiltonian_path_fixed_ends() {
        //! Test that [Graph::hamiltonian_path_fixed_ends] finds a Hamiltonian path with fixed start and end vertices in the graph.

        let graph = init_test_graph();
        let start_vertex = TEST_GRAPH_VERTICES[0];
        let end_vertex = TEST_GRAPH_VERTICES[1];
        let path = graph.hamiltonian_path_fixed_ends(&start_vertex, &end_vertex);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(*path.first().unwrap(), &start_vertex);
        assert_eq!(*path.last().unwrap(), &end_vertex);
        assert_eq!(
            path.iter().copied().collect::<HashSet<_>>().len(),
            TEST_GRAPH_SIZE
        );
        for i in 0..path.len() - 1 {
            let v1 = path[i];
            let v2 = path[i + 1];
            assert!(graph.edge(v1, v2).is_some());
        }
    }

    #[test]
    fn graph_hamiltonian_path_fixed_ends_nonexistent() {
        //! Test that [Graph::hamiltonian_path_fixed_ends] panics when either the start or end vertex is not present in the graph.

        let graph = init_test_graph();
        let start_vertex = Vertex::new("Z");
        let end_vertex = TEST_GRAPH_VERTICES[1];
        let result = catch_unwind(|| {
            graph.hamiltonian_path_fixed_ends(&start_vertex, &end_vertex);
        });
        assert!(result.is_err());

        let start_vertex = TEST_GRAPH_VERTICES[0];
        let end_vertex = Vertex::new("Z");
        let result = catch_unwind(|| {
            graph.hamiltonian_path_fixed_ends(&start_vertex, &end_vertex);
        });
        assert!(result.is_err());
    }

    #[test]
    fn graph_hamiltonian_path_fixed_ends_min() {
        //! Test that [Graph::hamiltonian_path_fixed_ends_min] finds the minimum cost Hamiltonian path with fixed start and end vertices in the graph.

        let graph = init_test_graph();
        let start_vertex = TEST_GRAPH_VERTICES[0];
        let end_vertex = TEST_GRAPH_VERTICES[3];
        let result = graph.hamiltonian_path_fixed_ends_min(&start_vertex, &end_vertex);
        assert!(result.is_some());
        let (cost, path) = result.unwrap();
        assert_eq!(cost, TEST_GRAPH_HAM_PATH_FIXED_ENDS_MIN_COST);
        assert_eq!(*path.first().unwrap(), &start_vertex);
        assert_eq!(*path.last().unwrap(), &end_vertex);
        assert_eq!(
            path.iter().copied().collect::<HashSet<_>>().len(),
            TEST_GRAPH_SIZE
        );
        // check that the cost is correct
        let mut calculated_cost = 0;
        for i in 0..path.len() - 1 {
            let v1 = path[i];
            let v2 = path[i + 1];
            calculated_cost += graph.edge(v1, v2).unwrap();
        }
        assert_eq!(cost, calculated_cost);
    }

    #[test]
    fn graph_hamiltonian_path_fixed_ends_min_nonexistent() {
        //! Test that [Graph::hamiltonian_path_fixed_ends_min] panics when either the start or end vertex is not present in the graph.

        let graph = init_test_graph();
        let start_vertex = Vertex::new("Z");
        let end_vertex = TEST_GRAPH_VERTICES[1];
        let result = catch_unwind(|| {
            graph.hamiltonian_path_fixed_ends_min(&start_vertex, &end_vertex);
        });
        assert!(result.is_err());

        let start_vertex = TEST_GRAPH_VERTICES[0];
        let end_vertex = Vertex::new("Z");
        let result = catch_unwind(|| {
            graph.hamiltonian_path_fixed_ends_min(&start_vertex, &end_vertex);
        });
        assert!(result.is_err());
    }

    #[test]
    fn graph_hamiltonian_path_fixed_ends_max() {
        //! Test that [Graph::hamiltonian_path_fixed_ends_max] finds the maximum cost Hamiltonian path with fixed start and end vertices in the graph.

        let graph = init_test_graph();
        let start_vertex = TEST_GRAPH_VERTICES[0];
        let end_vertex = TEST_GRAPH_VERTICES[3];
        let result = graph.hamiltonian_path_fixed_ends_max(&start_vertex, &end_vertex);
        assert!(result.is_some());
        let (cost, path) = result.unwrap();
        assert_eq!(cost, TEST_GRAPH_HAM_PATH_FIXED_ENDS_MAX_COST);
        assert_eq!(*path.first().unwrap(), &start_vertex);
        assert_eq!(*path.last().unwrap(), &end_vertex);
        assert_eq!(
            path.iter().copied().collect::<HashSet<_>>().len(),
            TEST_GRAPH_SIZE
        );
        // check that the cost is correct
        let mut calculated_cost = 0;
        for i in 0..path.len() - 1 {
            let v1 = path[i];
            let v2 = path[i + 1];
            calculated_cost += graph.edge(v1, v2).unwrap();
        }
        assert_eq!(cost, calculated_cost);
    }

    #[test]
    fn graph_hamiltonian_path_fixed_ends_max_nonexistent() {
        //! Test that [Graph::hamiltonian_path_fixed_ends_max] panics when either the start or end vertex is not present in the graph.

        let graph = init_test_graph();
        let start_vertex = Vertex::new("Z");
        let end_vertex = TEST_GRAPH_VERTICES[1];
        let result = catch_unwind(|| {
            graph.hamiltonian_path_fixed_ends_max(&start_vertex, &end_vertex);
        });
        assert!(result.is_err());

        let start_vertex = TEST_GRAPH_VERTICES[0];
        let end_vertex = Vertex::new("Z");
        let result = catch_unwind(|| {
            graph.hamiltonian_path_fixed_ends_max(&start_vertex, &end_vertex);
        });
        assert!(result.is_err());
    }
}
