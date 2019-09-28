pub mod node {
    use std::collections::HashMap;
    #[derive(Clone, Debug, PartialEq)]
    pub struct Node {
        pub label: String,
        pub attrs: HashMap<String, String>,
    }
    impl Node {
        pub fn new(label: &str) -> Self {
            Node {
                label: label.to_owned(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|&(k, v)| (k.to_owned(), v.to_owned()))
                .collect();
            self
        }

        pub fn get_attr(&self, attr: &str) -> Option<&str> {
            self.attrs.get(attr).map(String::as_str)
        }
    }
}

pub mod edge {
    use std::collections::HashMap;
    #[derive(Clone, Debug, PartialEq)]
    pub struct Edge {
        pub from: String,
        pub to: String,
        pub attrs: HashMap<String, String>,
    }
    impl Edge {
        pub fn new(from: &str, to: &str) -> Self {
            Edge {
                from: from.to_owned(),
                to: to.to_owned(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|&(k, v)| (k.to_owned(), v.to_owned()))
                .collect();
            self
        }
    }
}
