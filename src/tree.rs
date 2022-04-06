use std::fmt::{Debug, Write};

pub type Tree = Node;

pub struct NameScope {
    pub name: String,
    pub scope: usize
}

impl Debug for NameScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("(\"{}\", {})", self.name, self.scope).as_str())
    }
}

#[derive(Clone)]
pub struct Node {
    pub name: String,
    pub depth: usize,
    pub id: u128,
    pub children: Vec<Node>
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str()).unwrap();
        f.write_char('\n').unwrap();
        for (i, k) in self.children.clone().into_iter().enumerate() {
            let spaces = generate_spaces(self.depth);
            if i == self.children.len()-1 {
                f.write_str(format!("{spaces}└── {:?} {}", k, k.id).as_str()).unwrap()
            } else {
                f.write_str(format!("{spaces}├── {:?} {}", k, k.id).as_str()).unwrap();
            }
        }

        f.write_str("")
    }
}

fn generate_spaces(mul: usize) -> String {
    let mut spaces = String::new();
    for x in 0..4*mul {
        spaces.push(' ');
    }
    spaces
}


impl Node {
    pub fn new(name: &str, children: Option<Vec<Node>>) -> Self {
        return Self {
            name: name.to_string(),
            depth: 0,
            id: 0,
            children: children.unwrap_or(vec![])
        }
    }
    pub fn add_connection(&mut self, node: Node) {
        self.children.push(node);
    }
}