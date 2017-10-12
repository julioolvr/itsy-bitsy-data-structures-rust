use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::Display;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    edges: Vec<Rc<Edge<T>>>,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.value == other.value
    }
}

#[derive(Debug)]
pub struct Edge<T> {
    from: Weak<RefCell<Node<T>>>,
    to: Rc<RefCell<Node<T>>>,
}

impl<T> PartialEq for Edge<T> {
    fn eq(&self, other: &Edge<T>) -> bool {
        // 🤔🤔🤔🤔🤔
        self as *const Edge<T> == other as *const Edge<T>
    }
}

impl<T> Edge<T> {
    pub fn new(from: Weak<RefCell<Node<T>>>, to: Rc<RefCell<Node<T>>>) -> Edge<T> {
        Edge { from, to }
    }
}

pub struct Graph<T> {
    nodes: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T: PartialEq + Display> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph { nodes: Vec::<Rc<RefCell<Node<T>>>>::new() }
    }

    pub fn add_node(&mut self, value: T) {
        self.nodes
            .push(Rc::new(RefCell::new(Node {
                                           value,
                                           edges: vec![],
                                       })));
    }

    pub fn find(&self, value: &T) -> Option<Rc<RefCell<Node<T>>>> {
        for node in &self.nodes {
            if node.borrow().value == *value {
                return Some(node.clone());
            }
        }

        None
    }

    pub fn add_edge(&mut self, from: &T, to: &T) {
        let from_node = self.find(from).expect("Can\'t find node");
        let to_node = self.find(to).expect("Can\'t find node");
        let edge = Rc::new(Edge::new(Rc::downgrade(&from_node), to_node.clone()));

        from_node.borrow_mut().edges.push(edge.clone());
        to_node.borrow_mut().edges.push(edge.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_returns_none_if_no_node_with_the_given_value_exists() {
        let graph = Graph::<i32>::new();
        assert!(graph.find(&42).is_none());
    }

    #[test]
    fn add_node_and_find_it() {
        let mut graph = Graph::new();
        let elem = 42;
        graph.add_node(42);
        assert!(graph.find(&elem).is_some());
    }

    #[test]
    fn add_edge_and_find_related() {
        let mut graph = Graph::new();
        let first_elem = 42;
        let second_elem = 100;
        graph.add_node(first_elem);
        graph.add_node(second_elem);

        let first_node = graph.find(&first_elem).unwrap();
        let second_node = graph.find(&second_elem).unwrap();

        graph.add_edge(&first_elem, &second_elem);

        assert_eq!(first_node,
                   first_node.borrow().edges[0].from.upgrade().unwrap(),
                   "The `from` part of the edge must be a weak reference to the node it was added to");

        assert_eq!(second_node,
                   first_node.borrow().edges[0].to,
                   "The `to` part of the edge must be a strong reference to the other node");

        assert_eq!(first_node.borrow().edges[0],
                   second_node.borrow().edges[0],
                   "The second node must have the same edge that was added to the first one");
    }
}