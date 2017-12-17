use util::read;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Node {
    key: String,
    value: u32,
    parent: Option<Box<Node>>,
    children: Vec<String>,
    child_nodes: Option<Vec<Node>>,
}

pub fn solution() {
    let data = read("day7".to_owned());

    let mut root_tree: Vec<Node> = Vec::new();


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

            root_tree.push(Node { key: key.to_string(), value: value, parent: None, children: some, child_nodes: None });
            return key;
        })
        .collect::<Vec<String>>();


    let temp = root_tree.clone();

    for node in &mut root_tree {
//        set_child_nodes(node, &temp);
        set_parent_node(node, &temp);
    }

//    let mut parent_node = root_tree.iter().find(|&x| &x.key == key).unwrap().clone();
    let mut parent_node: Option<Node> = None;
    for node in root_tree {
        match node.parent {
            None => parent_node = Some(node),
            Some(x) => ()
        }
    }
//    let da_node = get_node(&"kiatxq".to_string(), &temp);
//    println!("{}", da_node.value);
//    1232 - 6 = 1226

    /*
    find diff of root nodes
    find leaf node where all values are equal
    leaf node value - diff
    */

//    let total = get_total(da_node, &temp);

//    for node_key in da_node.children {
//        let node = get_node(&node_key, &temp);
//        let total = get_total(node, &temp);
//        println!("{} total: {}", node_key, total);
//    }

    let mut value = 0;
    let mut diff = 0;
    for node_key in parent_node.unwrap().children {
        let node = get_node(&node_key, &temp);
        let total = get_total(node, &temp);
        if value == 0 {
            value = total;
        }
        if value != total {
            // check this node
            diff = total - value;
        }
        println!("{} total: {}", node_key, total);
    }

//    println!("{:?}", &parent_node);
}


fn get_node(key: &String, nodes: &Vec<Node>) -> Node {
    return nodes.iter().find(|&x| &x.key == key).unwrap().clone();
}

fn get_total(node: Node, nodes: &Vec<Node>) -> u32 {
    let mut sum = 0;
    sum += &node.value;
    if node.children.len() > 0 {
        for child in node.children {
            let child_node = get_node(&child, nodes);
            sum += get_total(child_node, nodes);
        }
    }
//    println!("{} total: {}", node.key, sum);
    return sum;
}

fn set_parent_node(child_node: &mut Node, nodes: &Vec<Node>) {
    for node in nodes {
        if node.children.contains(&child_node.key) {
            child_node.parent = Some(Box::new(node.clone()));
        }
    }
}

fn set_child_nodes(child_node: &mut Node, nodes: &Vec<Node>) {
    if child_node.children.len() > 0 {
        let mut new_nodes: Vec<Node> = Vec::new();
        for key in &child_node.children {
            let mut node = nodes.iter().find(|&x| &x.key == key).unwrap().clone();
            set_child_nodes(&mut node, nodes);
            new_nodes.push(node);
        }

        child_node.child_nodes = Some(new_nodes);
    }
}
