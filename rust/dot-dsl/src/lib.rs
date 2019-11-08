pub use std::collections::HashMap;

pub mod graph {
    use std::collections::HashMap;
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge<'a> {
                pub p1: &'a str,
                pub p2: &'a str,
                pub attrs: HashMap<&'a str, &'a str>,
            }

            impl<'a> Edge<'a> {
                pub fn new(p1: &'a str, p2: &'a str) -> Self {
                    Edge {
                        p1,
                        p2,
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    for (x, y) in attrs.iter() {
                        self.attrs.insert(x, y);
                    }
                    self
                }
            }

        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node<'a> {
                pub name: String,
                pub attrs: HashMap<&'a str, &'a str>,
            }

            impl<'a> Node<'a> {
                pub fn new(arg: &str) -> Self {
                    Node {
                        name: arg.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    for (x, y) in attrs.iter() {
                        self.attrs.insert(x, y);
                    }
                    self
                }

                pub fn get_attr(self, attr_name: &str) -> Option<&'a str> {
                    self.attrs.get(attr_name).cloned()
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Graph<'a> {
        pub nodes: Vec<graph_items::node::Node<'a>>,
        pub edges: Vec<graph_items::edge::Edge<'a>>,
        pub attrs: HashMap<String, String>,
    }

    impl<'a> Default for Graph<'a> {
        fn default() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph::default()
        }

        pub fn get_node(&self, n: &str) -> Option<graph_items::node::Node> {
            self.nodes.iter().find(|x| x.name == n).cloned()
        }

        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node<'a>]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &'a [graph_items::edge::Edge]) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (x, y) in attrs.iter() {
                self.attrs.insert(x.to_string(), y.to_string());
            }
            self
        }
    }
}
