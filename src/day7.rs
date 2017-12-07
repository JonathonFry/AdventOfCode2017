use util::read;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Node {
    key: String,
    value: u32,
    parent: Option<Box<Node>>,
    children: Vec<String>
}

struct Node1 {
    key: String,
    value: u32,
    parent: Option<Box<Node1>>,
    children: Vec<Node1>
}

struct Tree {
    nodes: HashMap<String, Tree>
}


pub fn solution() {
    let data = read("day7".to_owned());

    let mut root_tree: Vec<Node> = Vec::new();

    let mut tree = Tree { nodes: HashMap::new() };

    let vec = data.split("\n")
        .map(|x| {
            let tree: Vec<String> = x.split(" -> ").map(|s| s.to_string()).collect();
            let mut some: Vec<String> = Vec::new();
            if tree.len() > 1 {
                let mut sub: Vec<String> = tree[1].split(", ").map(|s| s.to_string()).collect();
                some.append(&mut sub);
            }

            let children: Vec<String> = x.split(" ").map(|s| s.to_string()).collect();
            let key = children[0].to_owned();
            let value = children[1].replace("(", "").replace(")", "").parse::<u32>().unwrap();

            root_tree.push(Node { key: key.to_string(), value: value, parent: None, children: some });
            return key;
        })
        .collect::<Vec<String>>();


    let mut root_nodes: Vec<Node1> = Vec::new();
    let mut temp = root_tree.clone();

    for node in &mut root_tree {
        set_parent_node(node, &temp);
    }

    for node in root_tree {
        match node.parent {
            None => println!("{}", node.key),
            Some(x) => ()
        }
    }
}


fn set_parent_node(child_node: &mut Node, nodes: &Vec<Node>) {
    for node in nodes {
        if node.children.contains(&child_node.key) {
            child_node.parent = Some(Box::new(node.clone()));
        }
    }
}
