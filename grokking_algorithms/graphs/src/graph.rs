
use std::collections::{ HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::cmp::PartialOrd;



#[derive(Debug, Clone)]
pub struct NotInGraph;

impl fmt::Display for NotInGraph{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"the node does not belong to the graph")
    }
}

pub trait MyGraph<'a,N,E>  
    where N: 'a +Eq + Hash + Clone ,
    E: 'a + PartialOrd + Clone   {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<&'a N, Vec<(&'a N, &'a E)>>;
    fn adjacency_table(&self) -> &HashMap<&'a N, Vec<(&'a N, &'a E)>>;

    fn add_node (&mut self, node:&'a N) -> bool{
        match self.adjacency_table().get(node) {
            None => {
                self.adjacency_table_mutable()
                .insert(node,Vec::new());
                true
            }
            _ => false
        }
    }
    fn add_edge(&mut self, edge: (&'a N, &'a N,  &'a E)){
        self.add_node(edge.0);
        self.add_node(edge.1);
        self.adjacency_table_mutable()
            .entry(edge.0)
            .and_modify(|e|{
                e.push((edge.1,edge.2))
            });
    }
    

    fn neighbours(&self, node: &'a N,) -> Result<&Vec<(&'a N, &'a E)>, NotInGraph>{
        match self.adjacency_table().get(node){
            None => Err(NotInGraph),
            Some(i) => Ok(i)
        }
    }
    fn contains(&self, node: &'a N) -> bool {
        self.adjacency_table().contains_key(node)
    }
    
    fn nodes(&self) -> HashSet<&'a N> {
       self.adjacency_table().keys().copied().collect()
       
    }
     

    fn edges(&self) ->  Vec<(&'a N, &'a N, &'a E)> {
       self.adjacency_table()
            .iter()
            .flat_map(|(from_node, from_node_neighbours)|{
                    from_node_neighbours
                        .iter()
                        .map(move |(to_node, weight)|  (*from_node, *to_node, *weight))
       })
       .collect()
    } 


}


pub struct DirectedGraph<'a,N,E> {
    adjacency_table: HashMap<&'a N, Vec<(&'a N,  &'a E)>> 
}

impl<'a,N, E> MyGraph<'a,N,E> for DirectedGraph<'a,N, E> 
where N: 'a +Eq + Hash + Clone ,
E: 'a + PartialOrd + Clone   {
    fn new() -> DirectedGraph<'a,N, E>  {
        DirectedGraph {
            adjacency_table: HashMap::new()
        }
    }
    fn adjacency_table_mutable(&mut self)-> &mut HashMap<&'a N, Vec<(&'a N, &'a E)>>{
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<&'a N, Vec<(&'a N, &'a E)>>{
        &self.adjacency_table
    }

}


pub struct UndirectedGraph <'a,N,E>{
    adjacency_table: HashMap<&'a N, Vec<(&'a N,  &'a E)>>,
}

impl<'a,N, E> MyGraph<'a,N, E> for UndirectedGraph<'a,N, E> 
where N: 'a +Eq + Hash + Clone ,
E: 'a + PartialOrd + Clone   {
    fn new() -> UndirectedGraph<'a, N, E>  {
        UndirectedGraph {
            adjacency_table: HashMap::new()
        }
    }
    fn adjacency_table_mutable(&mut self)-> &mut HashMap<&'a N, Vec<(&'a N, &'a E)>>{
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<&'a N, Vec<(&'a N, &'a E)>>{
        &self.adjacency_table
    }
 
    fn add_edge(&mut self, edge: (&'a N, &'a N,  &'a E)){
        self.add_node(edge.0);
        self.add_node(edge.1);
       
        self.adjacency_table_mutable()
            .entry(edge.0)
            .and_modify(|e|{
                e.push((edge.1,edge.2))
            });
        self.adjacency_table_mutable()
            .entry(edge.1)
            .and_modify(|e|{
                e.push((edge.0,edge.2))
            });
    } 
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node_directed() {
        let mut graph = DirectedGraph::<i32, i32>::new();
        assert!(graph.add_node(&1));
        assert!(graph.contains(&1));
        assert!(!graph.add_node(&1));  // Node 1 should already exist
    }

    #[test]
    fn test_add_edge_directed() {
        let mut graph = DirectedGraph::<i32, i32>::new();
        graph.add_node(&1);
        graph.add_node(&2);
        graph.add_edge((&1, &2, &10));

        let neighbours = graph.neighbours(&1).unwrap();
        assert_eq!(neighbours, &vec![(&2, &10)]);
    }

    #[test]
    fn test_add_node_undirected() {
        let mut graph = UndirectedGraph::<i32, i32>::new();
        assert!(graph.add_node(&1));
        assert!(graph.contains(&1));
        assert!(!graph.add_node(&1));  // Node 1 should already exist
    }

    #[test]
    fn test_add_edge_undirected() {
        let mut graph = UndirectedGraph::<i32, i32>::new();
        graph.add_node(&1);
        graph.add_node(&2);
        graph.add_edge((&1, &2, &10));

        let neighbours_1 = graph.neighbours(&1).unwrap();
        let neighbours_2 = graph.neighbours(&2).unwrap();
        assert_eq!(neighbours_1, &vec![(&2, &10)]);
        assert_eq!(neighbours_2, &vec![(&1, &10)]); // Edge should exist in both directions
    }

    #[test]
    fn test_graph_lifetime() {
        let node1 = 1;
        let node2 = 2;
        let weight = 10;

        // Create a scope to test lifetimes
        {
            // Create a graph within this scope
            let mut graph = DirectedGraph::<i32, i32>::new();

            // Add nodes and edges using references to `node1`, `node2`, and `weight`
            graph.add_node(&node1);
            graph.add_node(&node2);
            graph.add_edge((&node1, &node2, &weight));

            // Check if nodes and edges are correctly added
            assert!(graph.contains(&node1));
            assert!(graph.contains(&node2));
            let neighbours = graph.neighbours(&node1).unwrap();
            assert_eq!(neighbours, &vec![(&node2, &weight)]);
        }

        // At this point, `graph` goes out of scope and is dropped
        // If there were any lifetime issues, they would cause a compilation error
    }

    #[test]
    fn test_add_edge() {
        let mut graph: UndirectedGraph<String,f32> = UndirectedGraph::new();

        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");

        graph.add_edge((&a, &b, &5.0));
        graph.add_edge((&b, &c, &10.0));
        graph.add_edge((&c, &a, &7.0));

        let expected_edges = [
            (&a, &b, &5.0),
            (&b, &a, &5.0),
            (&c, &a, &7.0),
            (&a, &c, &7.0),
            (&b, &c, &10.0),
            (&c, &b, &10.0),
        ];
        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph: UndirectedGraph<String, i32> = UndirectedGraph::new();

        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");

        graph.add_edge((&a, &b, &5));
        graph.add_edge((&b, &c, &10));
        graph.add_edge((&c, &a, &7));

        assert_eq!(graph.neighbours(&a).unwrap(), &vec![(&b,&5), (&c, &7)]);
    }
    #[test]
    fn test_add_node() {
        let mut graph: DirectedGraph<String,i32> = DirectedGraph::new();

        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");

        graph.add_node(&a);
        graph.add_node(&b);
        graph.add_node(&c);

        assert_eq!(graph.nodes(), [&a, &b, &c].iter().cloned().collect());
    }

    #[test]
    fn test_add_node_with_struct() {
        #[derive(PartialEq, Eq, Hash, Debug, Clone)]
        struct Node {
            name: String,
            value: i32,
        }

        let mut graph: DirectedGraph<Node,i32> = DirectedGraph::new();

        let a = Node {
            name: String::from("a"),
            value: 1,
        };
        let b = Node {
            name: String::from("b"),
            value: 2,
        };
        let c = Node {
            name: String::from("c"),
            value: 3,
        };

        graph.add_node(&a);
        graph.add_node(&b);
        graph.add_node(&c);

        assert_eq!(graph.nodes(), [&a, &b, &c].iter().cloned().collect());
    }

    #[test]
    fn test_directed_graph_add_edge() {
        let mut graph: DirectedGraph<String,i32> = DirectedGraph::new();

        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");

        graph.add_edge((&a, &b, &5));
        graph.add_edge((&c, &a, &7));
        graph.add_edge((&b, &c, &10));

        let expected_edges = [(&a, &b, &5), (&c, &a, &7), (&b, &c, &10)];
        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }

    #[test]
    fn test_direct_graph_neighbours() {
        let mut graph: DirectedGraph<String,i32> = DirectedGraph::new();

        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");

        graph.add_edge((&a, &b, &5));
        graph.add_edge((&b, &c, &10));
        graph.add_edge((&c, &a, &7));

        assert_eq!(graph.neighbours(&a).unwrap(), &vec![(&b, &5)]);
    }

    #[test]
    fn test_contains() {
        let mut graph: DirectedGraph<String,i32> = DirectedGraph::new();

        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");
        let d = String::from("d");

        graph.add_node(&a);
        graph.add_node(&b);
        graph.add_node(&c);

        assert!(graph.contains(&a));
        assert!(graph.contains(&b));
        assert!(graph.contains(&c));
        assert!(!graph.contains(&d));
    }

}