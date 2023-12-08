use core::panic;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    pub directions: Vec<Direction>,
    pub nodes: HashMap<String, Node>,
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub left: String,
    pub right: String,
}

impl Map {
    pub fn parse(content: &str) -> Map {
        let mut lines = content.lines();
        let directions = Direction::parse(lines.next().unwrap());

        assert!(lines.next().unwrap().len() == 0);

        let mut nodes = HashMap::new();
        for node in lines.map(Node::parse) {
            nodes.insert(node.name.clone(), node);
        }

        Map { directions, nodes }
    }
}

impl Direction {
    pub fn parse(line: &str) -> Vec<Direction> {
        line.chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                x => panic!("Unknown direction {x}"),
            })
            .collect()
    }
}

impl Node {
    pub fn parse(line: &str) -> Node {
        let parts = line.split(" = ").collect::<Vec<_>>();
        let name = parts[0].into();

        let parts = parts[1].split(", ").collect::<Vec<_>>();
        let left = String::from(&parts[0][1..]);
        let right = String::from(&parts[1][..parts[1].len() - 1]);

        Node { name, left, right }
    }
}
