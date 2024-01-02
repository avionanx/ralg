use petgraph::{Undirected, Graph};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
pub fn get_cut_size(graph: &Graph<i32,(),Undirected>, partition_verts: &Vec<Vec<i32>>) -> u32{
    let mut count = 0;
    for edge in graph.edge_indices(){
        let (a,b) = graph.edge_endpoints(edge).unwrap();
        for i in 0..partition_verts.len(){
            if partition_verts[i].contains(&graph[a]) && !partition_verts[i].contains(&graph[b]) || partition_verts[i].contains(&graph[b]) && !partition_verts[i].contains(&graph[a]) {
                count += 1;
                break;
            }
        }
    }
    count
}

pub fn random_partition(graph: &Graph<i32,(),Undirected>, k: usize, seed: u64) -> Vec<Vec<i32>>{

    let mut partitions = vec![vec![]; k];
    let mut node_list: Vec<i32> = graph.node_indices().map(|x| graph[x]).collect();
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    node_list.shuffle(&mut rng);
    for i in 0..node_list.len(){
        partitions[i%k].push(node_list[i]);
    }
    
    partitions
}

pub fn calculate_gain(_graph: Graph<i32,(),Undirected>,_partition_a:Vec<i32>, _partition_b:Vec<i32> ){
    todo!();
}
pub fn swap_nodes(partition_a:&mut Vec<i32>, partition_b:&mut Vec<i32>, node_a: usize, node_b: usize){
    let temp_node:i32 = partition_a[node_a];
    partition_a[node_a] = partition_b[node_b];
    partition_b[node_b] = temp_node;
}
/// Minimize the number of edges between partitions
pub fn minimize_edges(graph: &Graph<i32,(),Undirected>, mut partitions:  Vec<Vec<i32>>,max_iter_count: usize) -> Vec<Vec<i32>>{
    let mut curr_cut_size = get_cut_size(graph, &partitions);
    let mut non_improving_iterations = 0;
    println!("Initial Cut Size: {:?}", curr_cut_size);
    for _ in 0..max_iter_count {
        // Select 2 random partitions from partition vector
        let (partition_index_a, partition_index_b) = {
            let mut rng = thread_rng();
            let a = rng.gen_range(0..partitions.len());
            let b = loop {
                let b_candidate = rng.gen_range(0..partitions.len());
                if b_candidate != a {
                    break b_candidate;
                }
            };
            (a, b)
        };
        // Select 2 random nodes from the 2 partitions. No seed here
        let (node_index_a, node_index_b) = {
            let mut rng = thread_rng();
            let a = rng.gen_range(0..partitions[partition_index_a].len());
            let b = rng.gen_range(0..partitions[partition_index_b].len());
            (
                partitions[partition_index_a][a],
                partitions[partition_index_b][b],
            )
        };
        // Save previous 2 partitions
        let (prev_partition_a, prev_partition_b) = (
            partitions[partition_index_a].clone(),
            partitions[partition_index_b].clone(),
        );
        // Swap nodes
        partitions[partition_index_a].retain(|&node| node != node_index_a);
        partitions[partition_index_b].retain(|&node| node != node_index_b);
        partitions[partition_index_a].push(node_index_b);
        partitions[partition_index_b].push(node_index_a);

        // Get cut size, if better, update cut size and partitions
        // If worse, revert to previous partitions
        let new_cut_size = get_cut_size(graph, &partitions);  
        if new_cut_size < curr_cut_size{
            println!("Better Cut Size: {:?}", new_cut_size);
            println!("Better Partitions: {:?}", &partitions);
            curr_cut_size = new_cut_size;
            non_improving_iterations = 0;
        } else {
            partitions[partition_index_a] = prev_partition_a;
            partitions[partition_index_b] = prev_partition_b;
            non_improving_iterations += 1;
        }

        // Stop if theres no improvement for 10 iterations or if cut size is 0 (cant get better)
        if non_improving_iterations >= max_iter_count || curr_cut_size == 0{
            break;
        }
    }
    partitions
}