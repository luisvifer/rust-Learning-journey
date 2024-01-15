use graphs::graph::*;
use std::fmt;
use std::collections::{VecDeque, HashSet, HashMap};

fn main() {
    println!("BFS!");
    let mut graph: DirectedGraph<String,i32> = DirectedGraph::new();
    let s = String::from("S");
    let a = String::from("A");
    let b = String::from("B");
    let c = String::from("C");
    let d = String::from("D");
    let f = String::from("F");
    graph.add_node(&s);
    graph.add_node(&a);
    graph.add_node(&b);
    graph.add_node(&c);
    graph.add_node(&d);
    graph.add_node(&f);

    graph.add_edge((&s,&a,&1));
    graph.add_edge((&s,&b,&1));
    graph.add_edge((&a,&c,&1));
    graph.add_edge((&b,&c,&1));
    graph.add_edge((&a,&f,&1));
    graph.add_edge((&b,&d,&1));
    graph.add_edge((&d,&f,&1));

    println!("Breadth First Search(BFS) from {:?} to {:?} :{:?}",&s,&f,breadth_first_search(&s, &graph, &f));

    match short_path_based_breadth_first_search(&s, &graph, &f)
    {   Ok(result) =>  println!("Short path BFS from {:?} to {:?} :{:?}",&s,&f,result),
        Err(err) => println!("{}",err)

    }   
    
}


fn breadth_first_search(origin:&String ,graph:&DirectedGraph<String,i32>, destination:&String) -> Vec<(String,i32)> {
    let mut searched: Vec<(String,i32)> = Vec::new();
    let mut queue: VecDeque<&String> = VecDeque::new();
    let mut visited: HashSet<&String> = HashSet::new();

     // Añade el nodo de origen a la cola y al conjunto de visitados
     queue.push_back(origin);
     visited.insert(origin);
 
    while let Some(node) = queue.pop_front() {
        
        if let Ok(neighbors) = graph.neighbours(node) {
            for &(neigh, weight) in  neighbors {
                if !visited.contains(neigh){
                    searched.push((neigh.clone(),*weight));
                    println!("{:?} compared to {:?}",neigh,destination);
                    if neigh == destination{
                        return  searched;
                    }else {
                        queue.push_back(neigh);
                        visited.insert(neigh);
                    }
            }
         }
            
        }
    }
  
    searched
}


pub struct NotGraph;

impl fmt::Display for NotGraph{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"There is no graph")
    }
}


fn short_path_based_breadth_first_search(
    origin: &String,
    graph: &DirectedGraph<String, i32>,
    destination: &String
) -> Result<Vec<String>, NotGraph> {
    let mut visited: HashMap<&String, &String> = HashMap::new();
    let mut queue: VecDeque<&String> = VecDeque::new();

    // Añade el nodo de origen a la cola
    queue.push_back(origin);
    visited.insert(origin, origin); // Marca el origen como visitado

    while let Some(node) = queue.pop_front() {
        if node == destination {
            // Reconstruye y devuelve el camino más corto cuando se encuentra el destino
            return Ok(reconstruct_path(origin, destination, &visited));
        }

        if let Ok(neighbors) = graph.neighbours(node) {
            for &(neigh, _) in neighbors {
                if !visited.contains_key(neigh) {
                    visited.insert(neigh, node);
                    queue.push_back(neigh);
                }
            }
        }
    }

    // Devuelve None si no se encuentra un camino
    Err(NotGraph)
}

fn reconstruct_path<'a>(
    origin: &'a String,
    destination: &'a String,
    predecessors: &HashMap<&'a String, &'a String>
) -> Vec<String> {
    let mut path = Vec::new();
    let mut current = destination;
    while current != origin {
        path.push(current.clone());
        current = predecessors[current];
    }
    path.push(origin.clone());
    path.reverse(); // El camino se construyó al revés, así que lo invertimos
    path
}