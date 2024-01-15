use graphs::graph::*;
use std::collections::{HashMap, HashSet};
fn main() {
    println!("Dijkstra's algorithm!");
    let mut graph: DirectedGraph<String, i32> = DirectedGraph::new();
    let s = String::from("S");
    let a = String::from("A");
    let b = String::from("B");
    let f = String::from("F");

    graph.add_node(&s);
    graph.add_node(&a);
    graph.add_node(&b);
    graph.add_node(&f);

    graph.add_edge((&s, &a, &6));
    graph.add_edge((&s, &b, &2));
    graph.add_edge((&a, &f, &1));
    graph.add_edge((&b, &a, &3));
    graph.add_edge((&b, &f, &5));

    println!(
        "Dijkstra's algorithm from {:?} to {:?} :{:?}",
        &s,
        &f,
        dijkstra(&s, &graph, &f)
    );
    let mut second_graph: DirectedGraph<String, i32> = DirectedGraph::new();
    let s = String::from("S");
    let a = String::from("A");
    let b = String::from("B");
    let c = String::from("C");
    let d = String::from("D");
    let f = String::from("F");

    second_graph.add_node(&s);
    second_graph.add_node(&a);
    second_graph.add_node(&b);
    second_graph.add_node(&c);
    second_graph.add_node(&d);
    second_graph.add_node(&f);

    second_graph.add_edge((&s, &a, &5));
    second_graph.add_edge((&s, &b, &2));
    second_graph.add_edge((&a, &c, &4));
    second_graph.add_edge((&a, &d, &2));
    second_graph.add_edge((&b, &a, &8));
    second_graph.add_edge((&b, &d, &7));
    second_graph.add_edge((&c, &d, &6));
    second_graph.add_edge((&c, &f, &3));
    second_graph.add_edge((&d, &f, &1));

    println!(
        "Dijkstra's algorithm from {:?} to {:?} :{:?}",
        &s,
        &f,
        dijkstra(&s, &second_graph, &f)
    );


}

fn dijkstra(
    origin: &String,
    graph: &DirectedGraph<String, i32>,
    destination: &String,
) -> Vec<(String, i32)> {
    let mut cost_list  = init_cost_table(origin, graph);
    let mut parent: HashMap<String, String> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();

    let mut current_node = find_lowest_cost_node(&mut cost_list,&visited);
    println!("{}",current_node.clone().unwrap_or(String::from("It's end!")));
    while current_node.is_some() && current_node != Some(destination.clone()) {
        if let Some(node) = &current_node {
          
        // Borrow cost_list mutably here
        if let Ok(neighbors) = graph.neighbours(node) {
            for &(ref neigh, weight) in neighbors {
                let new_cost = cost_list[node] + weight;
                if new_cost < cost_list[*neigh] {
                    cost_list.insert(neigh.to_string(), new_cost);
                    // Update parent map here if needed
                    parent.insert(neigh.to_string(), node.clone());
                }
            }
        }
        visited.insert(node.clone());
        }
    
        current_node = find_lowest_cost_node(&cost_list,&visited);
        println!("{}",current_node.clone().unwrap_or(String::from("It's end!")));
        
    }
    

    let mut path = Vec::new();
    let mut current = destination.clone();
    let  end = origin.clone();
    while current != end {
        if let Some(parent_node) = parent.get(&current) {
            path.push((current.clone(), cost_list[&current]));
            current = parent_node.clone();
        } else {
            // Path is not found (destination might be unreachable)
            return Vec::new();
        }
    }
    path.push((origin.clone(), cost_list[origin]));
    path.reverse(); // Reverse to get the path from origin to destination

    path
}



fn init_cost_table(
    init: &String,
    graph: &DirectedGraph<String, i32>,
) -> HashMap<String, i32> {
    let mut cost: HashMap<String, i32> = HashMap::new();
    for node in graph.nodes() {
        if (*node).ne(init){
            cost.insert(node.clone(), i32::MAX);
        }else {
            cost.insert(node.clone(), 0);
        }
    }

    cost
}

fn find_lowest_cost_node(costs: &HashMap<String, i32>,visited:&HashSet<String> ) -> Option<String> {
    let mut lowest_cost = i32::MAX;
    let mut lowest_cost_node:Option<String> = None;
    for (current_node,&current_cost) in costs{
        if current_cost < lowest_cost && !visited.contains(current_node) {
            lowest_cost = current_cost;
            lowest_cost_node = Some(current_node.clone());
        }

    }

    lowest_cost_node
}
