use crate::{problem, Solution};

// use hashbrown::HashMap;
// use petgraph::Graph;

pub struct Day16;

impl Solution for Day16 {
    fn name(&self) -> &'static str {
        "Proboscidea Volcanium"
    }

    fn part_a(&self) -> String {
        let _raw = problem::load(2022, 16);
        // let mut _graph = parse(&raw);

        // let out = dfs(&mut HashMap::new(), &graph, 0, 0, 0, 30);

        todo!()
    }

    fn part_b(&self) -> String {
        let _raw = problem::load(2022, 16);
        todo!()
    }
}

// struct World {
//     graph: Graph<Valve, ()>,
// }

// impl World {
//     fn new(raw: &str) -> Self {
//         Self { graph: parse(raw) }
//     }
// }

// struct Valve {
//     rate: u16,
//     opened: bool,
// }

// impl Valve {
//     fn new(rate: u16) -> Self {
//         Self {
//             rate,
//             opened: false,
//         }
//     }
// }

// fn parse(raw: &str) -> Graph<Valve, ()> {
//     let mut graph = Graph::new();
//     // maps (valve) -> (weight, to)
//     let mut working: HashMap<_, (u16, Vec<&str>)> = HashMap::new();
//     let mut nodes = HashMap::new();
//     for i in raw.lines() {
//         let parts = i.split_whitespace().collect::<Vec<_>>();
//         let name = parts[1];
//         let flow = parts[4]
//             .trim_start_matches("rate=")
//             .trim_end_matches(";")
//             .parse::<u16>()
//             .unwrap();
//         let to = parts[9..]
//             .iter()
//             .map(|x| x.trim_end_matches(','))
//             .collect::<Vec<_>>();

//         let node = graph.add_node(Valve::new(flow));
//         nodes.insert(name.to_string(), node);
//         working.insert(node, (flow, to));
//     }

//     for (node, (flow, to)) in working {
//         for i in to {
//             let to = nodes[i];
//             graph.add_edge(node, to, ());
//         }
//     }

//     graph
// }
