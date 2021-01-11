use graph_traits::{
	GraphBase, GraphEdgeAddable, GraphEdgeEndpoints, GraphEdgeFrom, GraphEdgeIndexable,
	GraphEdgeMutIndexable, GraphEdgeRemovable, GraphEdgeTo, GraphEdgesFrom, GraphNodeAddable,
	GraphNodeIndexable, GraphNodeMutIndexable, GraphNodeRemovable,
};

use std::{cmp::max, iter::Iterator, mem::replace};

/// Opaque struct which represents a node in the graph
#[derive(Debug, Clone, Copy)]
pub struct NodeID(usize);

/// Opaque struct which represents an edge in the graph
#[derive(Debug, Clone, Copy)]
pub struct EdgeID((usize, usize));

/// A simple directed graph implementation.
///
/// - Nodes are represented with `Vec<N>`
///
/// - Edges are represented with `Vec<Vec<Option<E>>>`
///
/// - All IDs are opaque data structures
///
/// Please see the trait implementations for more details
#[derive(Clone, Debug)]
pub struct SimpleGraph<N, E> {
	nodes: Vec<Option<N>>,
	edges: Vec<Vec<Option<E>>>,
}

impl<N, E> SimpleGraph<N, E> {
	pub fn new() -> Self {
		Self {
			nodes: Vec::new(),
			edges: Vec::new(),
		}
	}
}

impl<N, E> GraphBase for SimpleGraph<N, E> {
	type NodeID = NodeID;
	type EdgeID = EdgeID;
}

impl<N, E> GraphNodeAddable<N> for SimpleGraph<N, E> {
	/// Identical data added twice will return different node IDs
	fn add_node(&mut self, data: N) -> Self::NodeID {
		self.nodes.push(Some(data));
		NodeID(self.nodes.len() - 1)
	}
}

impl<N, E> GraphEdgeAddable<E> for SimpleGraph<N, E>
where
	E: Clone,
{
	/// Data added from node `A` to `B` will overwrite the data that was previously between those edges
	fn add_edge(&mut self, a: Self::NodeID, b: Self::NodeID, data: E) -> Self::EdgeID {
		let (a, b) = (a.0, b.0);

		let max_id = max(a, b);

		// Make sure that the vectors are large enough to contain the node IDs
		if self.edges.len() <= max_id {
			self.edges.resize(max_id + 1, Vec::new());
		}
		if self.edges[a].len() <= b {
			self.edges[a].resize(b + 1, None);
		}

		self.edges[a][b] = Some(data);
		EdgeID((a, b))
	}
}

impl<N, E> GraphNodeRemovable<N> for SimpleGraph<N, E> {
	/// Panics if the node is not in the graph
	///
	/// Panics if any edges lead into or out of that edge
	fn remove_node(&mut self, NodeID(id): Self::NodeID) -> N {
		// Panic if any edges point to or from the removed node
		for to_node in &self.edges[id] {
			assert!(to_node.is_none());
		}
		for from_node in &self.edges {
			assert!(from_node[id].is_none());
		}

		// Swap out the node
		replace(&mut self.nodes[id], None).unwrap()
	}
}

impl<N, E> GraphEdgeRemovable<E> for SimpleGraph<N, E> {
	/// Panics if the node is not in the graph
	fn remove_edge(&mut self, EdgeID((id_a, id_b)): Self::EdgeID) -> E {
		replace(&mut self.edges[id_a][id_b], None).unwrap()
	}
}

impl<N, E> GraphNodeIndexable<N> for SimpleGraph<N, E> {
	/// Get the data associated with a node
	///
	/// Panics if the node is not in the graph
	fn node(&self, NodeID(id): Self::NodeID) -> &N { self.nodes[id].as_ref().unwrap() }
}

impl<N, E> GraphNodeMutIndexable<N> for SimpleGraph<N, E> {
	/// Get the data associated with a node
	///
	/// Panics if the node is not in the graph
	fn node_mut(&mut self, NodeID(id): Self::NodeID) -> &mut N { self.nodes[id].as_mut().unwrap() }
}

impl<N, E> GraphEdgeIndexable<E> for SimpleGraph<N, E> {
	/// Get the data associated with an edge
	///
	/// Panics if the edge is not in the graph
	fn edge(&self, EdgeID((id_a, id_b)): Self::EdgeID) -> &E {
		self.edges[id_a][id_b].as_ref().unwrap()
	}
}

impl<N, E> GraphEdgeMutIndexable<E> for SimpleGraph<N, E> {
	/// Get the data associated with an edge
	///
	/// Panics if the edge is not in the graph
	fn edge_mut(&mut self, EdgeID((id_a, id_b)): Self::EdgeID) -> &mut E {
		self.edges[id_a][id_b].as_mut().unwrap()
	}
}

impl<N, E> GraphEdgeTo for SimpleGraph<N, E> {
	/// Find the destination of an edge
	fn edge_to(&self, EdgeID((_, id_b)): Self::EdgeID) -> Self::NodeID { NodeID(id_b) }
}

impl<N, E> GraphEdgeFrom for SimpleGraph<N, E> {
	/// Find the source of an edge
	fn edge_from(&self, EdgeID((id_a, _)): Self::EdgeID) -> Self::NodeID { NodeID(id_a) }
}

impl<N, E> GraphEdgeEndpoints for SimpleGraph<N, E> {}

impl<N, E> GraphEdgesFrom for SimpleGraph<N, E> {
	type EdgesFromOutput = Vec<EdgeID>;

	/// Find all edges from a node
	///
	/// Return result in a `Vec`
	fn edges_from(&self, NodeID(id): Self::NodeID) -> Self::EdgesFromOutput {
		self.edges[id]
			.iter()
			.enumerate()
			.filter_map(|(i, dest)| match dest {
				Some(_) => Some(EdgeID((id, i))),
				None => None,
			})
			.collect()
	}
}
