use std::{
    fs,
    io::{self, BufRead},
    net::ToSocketAddrs,
    process::Command,
};

use network_vis::{edge_options::EdgeOptions, network::Network, node_options::NodeOptions};
use tree::Tree;

use crate::tree::{NameScope, Node};

pub mod tree;

fn main() {
    let mut input = String::new();
    if cfg!(target_os = "windows") {
        input = String::from_utf8(
            Command::new("cmd")
                .args(["/C", "cargo tree"])
                .output()
                .expect("failed to execute process")
                .stdout
                .to_vec(),
        )
        .unwrap();
    } else {
        input = String::from_utf8(
            Command::new("sh")
                .args(["-c", "cargo tree"])
                .output()
                .expect("failed to execute process")
                .stdout
                .to_vec(),
        )
        .unwrap();
    };

    input = replace_with(input, vec!["└", "├", "─", "│"], " ");
    input = replace_with(input, vec!["    "], "=");
    let mut input = input
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if input[0] == "" {
        input.remove(0);
    }
    input[0] = input[0].split(" ").collect::<Vec<_>>()[0].to_string();
    if input[input.len() - 1] == "" {
        input.remove(input.len() - 1);
    }
    println!("{input:#?}");

    turn_into_tree(input);
}

fn replace_with(mut from: String, chars: Vec<&str>, with: &str) -> String {
    for x in chars {
        from = from.replace(x, with);
    }
    return from;
}

fn turn_into_tree(input: Vec<String>) {
    let mut net = Network::new();
    let tree_input = input
        .iter()
        .map(|x| NameScope {
            name: x.clone().replace("=", ""),
            scope: x.matches("=").count(),
        })
        .collect::<Vec<_>>();
    println!("{tree_input:#?}");
    let flat = tree_input
        .into_iter()
        .map(|x| (x.name, x.scope))
        .collect::<Vec<_>>();
    let results = to_tree(flat);
    println!("{:#?}", results);

    net.add_node(
        results.id,
        results.name.as_str(),
        Some(vec![NodeOptions::Hex("#fca9a9")]),
    );
    make_connections(results, &mut net);

    net.create("tree.html").unwrap();
}

fn to_tree(flat: Vec<(String, usize)>) -> Node {
    let mut stack = Vec::new();
    let mut id = 0;
    for (name, depth) in flat.into_iter().rev() {
        let mut children = Vec::new();
        while let Some((prev, prev_depth)) = stack.pop() {
            // We're a parent.
            if depth < prev_depth {
                children.push(prev);
            } else {
                stack.push((prev, prev_depth));
                break;
            }
        }

        stack.push((
            Node {
                name,
                depth,
                id,
                children,
            },
            depth,
        ));
        id += 1;
    }

    assert!(stack.len() == 1);
    stack.pop().unwrap().0
}

fn make_connections(results: Node, net: &mut Network) {
    for k in results.clone().children {
        net.add_node(k.id, k.name.as_str(), None);
        net.add_edge(results.clone().id, k.id, None, true);
        make_connections(k.clone(), net);
    }
}
