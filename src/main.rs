use petgraph::graph::UnGraph;

use alg::{random_partition, get_cut_size, minimize_edges};
fn main() {

    // Create an undirected graph
    let mut g = UnGraph::<i32, ()>::default();
    let k = 3;
    // Add nodes with weights
    let node1 = g.add_node(1);
    let node2 = g.add_node(2);
    let node3 = g.add_node(3);
    let node4 = g.add_node(4);
    let node5 = g.add_node(5);
    let node6 = g.add_node(6);
    // Add edges between nodes
    g.add_edge(node1, node2, ());
    g.add_edge(node2, node3, ());
    g.add_edge(node4, node5, ());
    g.add_edge(node5, node6, ());
    let div = random_partition(&g, k, 1);
    let div = minimize_edges(&g, div, 100);
    let cutsize = get_cut_size(&g, &div);
    //let cutsize = get_cut_size(&g,&div);
    println!("Result: {:?}, Partitions: {:?}", cutsize, &div);
}
