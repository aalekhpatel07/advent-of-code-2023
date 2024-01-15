// use petgraph::{visit::{Visitable, VisitMap, IntoEdges, EdgeRef, TrackPath, TrackablePath, GraphBase}, algo::Measure};
// use std::hash::Hash;
// use petgraph;

// fn all_simple_paths_helper<G, F, D, IsGoal, PathVisitor>(
//     graph: G,
//     start: G::NodeId,
//     prev: Option<G::NodeId>,
//     is_goal: &mut IsGoal,
//     edge_cost: &mut F, 
//     visited: &mut <G as Visitable>::Map,
//     path_tracker: &mut <G as TrackablePath>::Tracker,
//     dist: D,
//     visitor: &mut PathVisitor
// ) 
// where
//     G: Visitable + IntoEdges,
//     IsGoal: FnMut(G::NodeId) -> bool,
//     D: Measure + Copy,
//     PathVisitor: FnMut(Vec<G::NodeId>, D),
//     F: FnMut(G::EdgeRef) -> D,
//     G::NodeId: std::hash::Hash + Eq,
// {
//     visited.visit(start);
//     path_tracker.set_predecessor(start, prev);

//     if is_goal(start) {
//         visitor(path_tracker.reconstruct_path_to(start), dist);
//     } else {
//         for edge in graph.edges(start) {
//             let neighbor = edge.target();
//             let edge_weight = edge_cost(edge);

//             if !visited.is_visited(&neighbor) {
//                 all_simple_paths_helper(
//                     graph, 
//                     neighbor,
//                     Some(start),
//                     is_goal, 
//                     edge_cost, 
//                     visited, 
//                     path_tracker, 
//                     dist + edge_weight, 
//                     visitor
//                 )
//             }
//         }
//     }
//     path_tracker.unset_predecessor(start);
//     visited.unvisit(start);
// }

// pub fn all_simple_paths<G, F, D, IsGoal, PathVisitor>(
//     graph: G,
//     start: G::NodeId,
//     mut is_goal: IsGoal,
//     mut edge_cost: F, 
//     mut visitor: PathVisitor
// ) 
// where
//     G: IntoEdges + Visitable + TrackablePath,
//     IsGoal: FnMut(G::NodeId) -> bool,
//     D: Measure + Copy,
//     PathVisitor: FnMut(Vec<G::NodeId>, D),
//     F: FnMut(G::EdgeRef) -> D,
//     G::NodeId: Hash + Eq,
// {
//     let mut visited = graph.visit_map();
//     let mut path_tracker = graph.path_tracker();

//     all_simple_paths_helper(
//         graph, 
//         start,
//         None,
//         &mut is_goal, 
//         &mut edge_cost, 
//         &mut visited, 
//         &mut path_tracker,
//         D::default(), 
//         &mut visitor
//     )
// }

// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;

//     use super::*;
//     use petgraph::Graph;

//     #[test]
//     pub fn all_paths() {
//         let mut graph: Graph<(i32, i32), i32, petgraph::Undirected, u32> = Graph::new_undirected();
//         let a = graph.add_node((0, 0));
//         let b = graph.add_node((0, 0));
//         let c = graph.add_node((0, 0));
//         let d = graph.add_node((0, 0));
//         let e = graph.add_node((0, 0));
//         let f = graph.add_node((0, 0));
//         let g = graph.add_node((0, 0));

//         graph.extend_with_edges(&[
//             (a, b, 1),
//             (a, f, 1),
//             (b, e, 1),
//             (b, c, 2),
//             (e, d, 1),
//             (f, g, 1),
//             (g, d, 1),
//             (c, d, 1)
//         ]);

//         let mut paths: HashMap<i32, Vec<Vec<_>>> = Default::default();

//         all_simple_paths(
//             &graph, 
//             a, 
//             |node| node == d, 
//             |e| *e.weight(),
//             |ref path, dist| {
//                 paths
//                 .entry(dist)
//                 .and_modify(
//                     |paths_so_far| {
//                         paths_so_far.push(path.iter().copied().collect());
//                     }
//                 ).or_insert(vec![path.iter().copied().collect()]);
//             }
//         );

//         assert_eq!(paths.get(&4).unwrap().len(), 1);
//         assert_eq!(paths.get(&3).unwrap().len(), 2);

//         println!("{:?}", paths.get(&4).unwrap()[0]);
//         println!("{:?}", paths.get(&3).unwrap()[0]);
//         println!("{:?}", paths.get(&3).unwrap()[1]);

//     }

// }