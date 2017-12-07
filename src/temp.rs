use util::read;
use std::collections::HashMap;
use std::collections::HashSet;

/*
Tower=
key
value
sub towers
*/

/*
Step 1: Generate list of Towers
Step 2: Iterate through towers and build tree
*/
/*
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
*/

struct Tower {
    children: Vec<Tower>
}


struct Level {
    level: HashMap<String, Level>
}

pub fn solution() {
    let data = read("day7".to_owned());

    let tower: Program = Program { tower: HashMap::new(), value: 0 };

    let mut level = Level { level: HashMap::new() };

    let mut items: HashMap<&str, Vec<&str>> = HashMap::new();

    let vec = data.split("\n")
        .map(|x| {
            let tree: Vec<&str> = x.split(" -> ").collect();
            let mut some: Vec<&str> = Vec::new();
            if tree.len() > 1 {
                let mut sub: Vec<&str> = tree[1].split(", ").collect();
                some.append(&mut sub);
            }


            let v: Vec<&str> = x.split(" ").collect();
            let key = v[0];
            let value = v[1].replace("(", "").replace(")", "").parse::<u32>().unwrap();
            items.insert(key, some);
            key
        })
        .collect::<Vec<&str>>();

    let mut structure: Tower = Tower { children: Vec::new() };

    generate_tree(structure, items);

    for (key, value) in items {
        let mut s = String::new();
        for sub in value {
            s.push_str(&sub);
            s.push_str(" ");
        }
        println!("{}, {}", key, s);
    }
}

fn generate_tree(tower: Tower, tree: HashMap<&str, Vec<&str>>) {
    for (key, value) in tree {
        if value.len() > 0 {
            for sub in value {
                generate_tree()
            }
        } else {
            tower.children.push(value)
        }
    }
}

struct Program {
    tower: HashMap<String, Program>,
    value: u32
}


/*
---------
*/

use util::read;
use std::collections::HashMap;
use std::collections::HashSet;

struct Node {
    key: String,
    value: u32,
    parent: Box<Option<Node>>,
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

//#[derive(PartialEq, Eq, Hash)]
//struct Node1 {
//    key: String,
//    value: u32,
//}

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

            root_tree.push(Node { key: key.to_string(), value: value, parent: Box::new(None), children: some });
            return key;
        })
        .collect::<Vec<String>>();


    let mut root_nodes: Vec<Node1> = Vec::new();

    for node in root_tree {
        set_parent_node(node, root_tree);
    }

//    root_tree.sort_by(|a, b| b.children.len().cmp(&a.children.len()));


//    for node in root_tree {
//        let mut children = String::new();
//        if node.children.len() > 0 {
//            for child in node.children {
//                children.push_str(&child);
//                children.push_str(" ");
//            }
//        }
//        println!("{} {} {}", node.key, node.value, children);
//    }

//    let filtered: Vec<Node> = root_tree.into_iter()
//        .filter(|x| x.children.len() > 0)
//        .collect();
//    find_root(&filtered);
}

/*
                gyxo
              /
         ugml - ebii
       /      \
      |         jptl
      |
      |         pbga
     /        /
tknk --- padx - havc
     \        \
      |         qoyq
      |
      |         ktlj
       \      /
         fwft - cntj
              \
                xhth
*/

/*
fwft 72 ktlj cntj xhth
padx 45 pbga havc qoyq
tknk 41 ugml padx fwft
ugml 68 gyxo ebii jptl
pbga 66
xhth 57
ebii 61
havc 66
ktlj 57
qoyq 66
jptl 61
gyxo 61
cntj 57
*/

fn set_parent_node(child_node: Node, nodes: Vec<Node>) {
    for node in nodes {
        if node.children.contains(&child_node.key) {
            child_node.parent = Box::new(Some(node));
        }
    }
}

//fn get_node(key: String, nodes: &Vec<Node1>) -> Node1 {
//    for node in nodes {
//        if node.key == key {
//            return node;
//        }
//    }
//}