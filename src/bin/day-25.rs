use petgraph::{
    algo::minimum_cut,
    graph::UnGraph,
    stable_graph::IndexType,
    visit::{
        Bfs, EdgeRef, GraphBase, IntoEdgeReferences, IntoEdges, IntoNodeIdentifiers,
        NodeCompactIndexable, NodeCount, NodeIndexable,
    },
    Graph, Undirected,
};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn main() {
    let data = include_str!("../../data/25.in");

    let graph = build_graph(data);
    println!("part 1: {}", solve(&graph));
}

pub fn build_graph(data: &str) -> petgraph::Graph<&str, (), Undirected> {
    let mut hmap = HashMap::<&str, Vec<_>>::new();

    data.lines().for_each(|line| {
        let (source, neighbors) = line.split_once(": ").unwrap();

        let neighbor_ids = neighbors.trim().split_whitespace();

        hmap.entry(source)
            .and_modify(|_| panic!("repeat entries?"))
            .or_insert(neighbor_ids.collect());
    });

    let mut unique_nodes = HashSet::new();
    let mut node_ids_to_names = HashMap::new();
    let mut node_names_to_ids = HashMap::new();

    hmap.iter().for_each(|(&source, neighbors)| {
        unique_nodes.insert(source);
        for neighbor in neighbors {
            unique_nodes.insert(neighbor);
        }
    });

    // ughh book-keeping.
    let mut graph = UnGraph::default();

    for node in unique_nodes {
        let node_id = graph.add_node(node);
        node_names_to_ids.insert(node, node_id);
        node_ids_to_names.insert(node_id, node);
    }

    hmap.iter().for_each(|(&source, neighbors)| {
        let source_id = *node_names_to_ids.get(&source).unwrap();
        for neighbor in neighbors {
            let neighbor_id = *node_names_to_ids.get(neighbor).unwrap();
            graph.add_edge(source_id, neighbor_id, ());
        }
    });

    graph
}

pub fn get_connected_components<G>(graph: G) -> Vec<Vec<G::NodeId>>
where
    G: GraphBase + NodeCount + IntoEdgeReferences + NodeIndexable + IntoNodeIdentifiers,
    G::NodeId: Hash + Eq,
{
    let mut vertex_sets = petgraph::unionfind::UnionFind::new(graph.node_count());
    for edge in graph.edge_references() {
        let (a, b) = (edge.source(), edge.target());
        vertex_sets.union(graph.to_index(a), graph.to_index(b));
    }

    let labels = vertex_sets.into_labeling();

    let mut label_map = HashMap::<usize, Vec<G::NodeId>>::new();
    for (&label, node_id) in labels.iter().zip(graph.node_identifiers()) {
        label_map
            .entry(label)
            .and_modify(|component| component.push(node_id))
            .or_insert(vec![node_id]);
    }

    label_map.values().cloned().collect::<Vec<_>>()
}

pub fn solve(graph: &Graph<&str, (), Undirected>) -> usize {
    let (edges, _) = minimum_cut(graph, |_| 1);
    let mut graph = graph.clone();
    assert_eq!(edges.len(), 3);

    for edge in edges {
        let (start, end) = graph.edge_endpoints(edge).unwrap();
        println!("Disconnect wires: {}/{}", &graph[start], &graph[end]);
        graph.remove_edge(edge);
    }

    let components = get_connected_components(&graph);
    assert_eq!(components.len(), 2);
    components[0].len() * components[1].len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smol() {
        let data = r"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

        let graph = build_graph(data);
        assert_eq!(solve(&graph), 9 * 6);
    }
}
