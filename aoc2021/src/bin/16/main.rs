// 12
use aoc2021::utils;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
enum NodeType {
    Start,
    End,
    Major,
    Minor,
}

#[derive(Debug, PartialEq, Clone)]
struct Node {
    name: String,
    node_type: NodeType,
}
impl<'a> FromStr for Node {
    type Err = ();
    fn from_str(node_name: &str) -> Result<Self, Self::Err> {
        Ok(Node {
            name: node_name.to_owned(),
            node_type: match node_name {
                "start" => NodeType::Start,
                "end" => NodeType::End,
                _ => {
                    if node_name.chars().next().unwrap().is_ascii_uppercase() {
                        NodeType::Major
                    } else {
                        NodeType::Minor
                    }
                }
            },
        })
    }
}
type NodeList = HashMap<String, Node>;
type NodeConnections<'a> = HashMap<&'a str, Vec<&'a Node>>;

struct PuzzleInput<'a> {
    connections: NodeConnections<'a>,
    nodes: NodeList,
}

fn part_1(input: &PuzzleInput) -> Option<i64> {
    None
}

fn parse_input_line(input_line: &String, graph: &mut PuzzleInput) {
    let nodes_split: Vec<&str> = input_line.split('c').collect();
    let node_a_key = nodes_split[0];
    let node_b_key = nodes_split[1];
    let node_a = node_a_key.parse::<Node>().unwrap();
    let node_b = node_b_key.parse::<Node>().unwrap();
    graph.nodes.insert(node_a_key.to_owned(), node_a);
    graph.nodes.insert(node_b_key.to_owned(), node_b);

    if let Some(con_vec) = graph.connections.get_mut(node_a.name.as_str()) {
        con_vec.push(graph.nodes.get(node_b_key).unwrap());
    } else {
        graph.connections.insert(&node_a.name, vec![&node_b]);
    }
}

fn main() {
    let lines_str = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let mut puzzle_input = PuzzleInput {
        nodes: HashMap::new(),
        connections: HashMap::new(),
    };
    lines_str
        .iter()
        .for_each(|line| parse_input_line(line, &mut puzzle_input));
    utils::measure_and_print(1, part_1, &puzzle_input);
    // utils::measure_and_print(2, part_2, &puzzle_input);
}
