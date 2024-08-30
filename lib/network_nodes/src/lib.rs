use std::{collections::BTreeMap, num::TryFromIntError, ops::Index, str::FromStr};

/// Error type for parsing the network
#[derive(Debug)]
pub enum NetworkError {
    ParseNode(String),
    ParseNodeAddr(String),
    GetSequence,
    GetNode(String),
    GetNodeAddr(String),
    InvalidInstruction(char),
    InvalidNodeName(String),
    TooManyAddresses(Vec<Node>),
    ConvertUsize(TryFromIntError),
}

impl std::error::Error for NetworkError {}

impl From<TryFromIntError> for NetworkError {
    fn from(value: TryFromIntError) -> Self {
        Self::ConvertUsize(value)
    }
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseNode(node) => write!(f, "Failed to parse node: {node}"),
            Self::ParseNodeAddr(node) => write!(f, "Failed to parse node address: {node}"),
            Self::GetSequence => write!(f, "Failed to get sequence"),
            Self::GetNode(node) => write!(f, "Failed to get node: {node}"),
            Self::GetNodeAddr(node) => write!(f, "Failed to get node address: {node}"),
            Self::InvalidInstruction(inst) => write!(f, "Invalid instruction: {inst}"),
            Self::InvalidNodeName(name) => write!(f, "Invalid node name: {name}"),
            Self::TooManyAddresses(nodes) => write!(f, "Too many addresses: {nodes:?}"),
            Self::ConvertUsize(int_err) => write!(f, "Failed to convert integer: {int_err:?}"),
        }
    }
}

/// An instruction for the network
#[derive(Debug)]
pub enum Instruction {
    Right,
    Left,
}

impl TryFrom<char> for Instruction {
    type Error = NetworkError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Self::Right),
            'L' => Ok(Self::Left),
            invalid => Err(NetworkError::InvalidInstruction(invalid)),
        }
    }
}

/// A node in the network
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node(String);

impl Node {
    /// Check if the node ends with a specific character
    #[inline(always)]
    pub fn ends_with(&self, char: char) -> bool {
        self.0
            .chars()
            .last()
            .expect("Nodes should never have an empty string as a name")
            == char
    }
}

impl FromStr for Node {
    type Err = NetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_alphabetic() || c.is_numeric()) {
            return Ok(Self(s.to_string()));
        }

        Err(NetworkError::InvalidNodeName(s.to_string()))
    }
}

/// An address for a node
pub struct NodeAddress([Node; 2]);

// Implement indexing for the NodeAddress, so that we can access the nodes with the instructions easily
impl Index<&Instruction> for NodeAddress {
    type Output = Node;

    #[inline(always)]
    fn index(&self, index: &Instruction) -> &Self::Output {
        match index {
            Instruction::Left => self.0.first().expect("Index 'Left' out of bounds"),
            Instruction::Right => self.0.last().expect("Index 'Right' out of bounds"),
        }
    }
}

impl FromStr for NodeAddress {
    type Err = NetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nodes = s
            .trim_matches(&['(', ')'])
            .split(", ")
            .map(Node::from_str)
            .collect::<Result<Vec<Node>, NetworkError>>()?;

        if nodes.len() != 2 {
            return Err(NetworkError::TooManyAddresses(nodes));
        }

        Ok(Self([nodes[0].clone(), nodes[1].clone()]))
    }
}

/// A sequence of instructions
#[derive(Debug)]
pub struct Sequence(Vec<Instruction>);

impl Sequence {
    /// Get the length of the sequence
    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'i> IntoIterator for &'i Sequence {
    type Item = &'i Instruction;
    type IntoIter = std::slice::Iter<'i, Instruction>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FromStr for Sequence {
    type Err = NetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .chars()
            .map(Instruction::try_from)
            .collect::<Result<Vec<Instruction>, NetworkError>>()?;

        Ok(Self(instructions))
    }
}

/// A network of nodes, with a sequence of instructions
pub struct Network {
    nodes: BTreeMap<Node, NodeAddress>,
    seq: Sequence,
}

impl Network {
    /// Find the amount of steps required to get to a specific node, using the sequence of instructions
    pub fn find_steps_required_for(&self, find: &Node) -> Result<u64, NetworkError> {
        let mut steps = 0;
        let mut current_node = self
            .nodes
            .keys()
            .next()
            .ok_or_else(|| NetworkError::GetNode("Failed getting first node".to_string()))?;

        while current_node != find {
            for inst in &self.seq {
                let address = self.nodes.get(current_node).ok_or_else(|| {
                    NetworkError::GetNode(format!("Failed getting node while stepping: {current_node:?}"))
                })?;

                steps += 1;

                current_node = &address[inst]
            }
        }

        Ok(steps)
    }

    /// Find the amount of "ghost steps" required to get to a specific node, using the sequence of instructions
    pub fn find_ghost_steps_required_for(&self, node_ends_with: char) -> Result<u64, NetworkError> {
        let mut cycles = 0;
        let mut current_nodes = self
            .nodes
            .keys()
            .filter(|node| node.ends_with('A'))
            .collect::<Vec<&Node>>();
        let mut node_cycles: Vec<u64> = Vec::new();

        while !current_nodes.iter().all(|node| node.ends_with(node_ends_with)) {
            for inst in &self.seq {
                current_nodes.iter_mut().try_for_each(|current| {
                    let address = self.nodes.get(current).ok_or_else(|| {
                        NetworkError::GetNode(format!("Failed getting node while stepping: {current:?}"))
                    })?;

                    *current = &address[inst];

                    Ok::<_, NetworkError>(())
                })?;
            }
            // Add to the cycle tracker
            cycles += 1;

            // Grab the amount of nodes we are tracking right now
            let current_nodes_len = current_nodes.len();

            // Remove nodes that end up at the right location after cycling
            current_nodes.retain(|node| !node.ends_with(node_ends_with));

            // If the old length is over than the current length, we found another cycle. Append it to the list
            if current_nodes_len > current_nodes.len() {
                node_cycles.push(cycles);
            }
        }

        // Find the LCM (Least Common Multiple) between all the cycles
        let lcm = node_cycles
            .iter()
            .fold(1u64, |least, common| Self::find_lcm(least, *common));

        let seq_len: u64 = self.seq.len().try_into()?;

        Ok(lcm * seq_len)
    }

    /// Find the greatest common factor between two numbers
    #[inline(always)]
    fn find_greatest_common_factor(a: u64, b: u64) -> u64 {
        let min_val = a.min(b);
        let max_val = a.max(b);

        if min_val == 0 {
            return max_val;
        }

        Self::find_greatest_common_factor(min_val, max_val % min_val)
    }

    /// Find the least common multiple between two numbers
    #[inline(always)]
    fn find_lcm(a: u64, b: u64) -> u64 {
        a * (b / Self::find_greatest_common_factor(a, b))
    }
}

impl FromStr for Network {
    type Err = NetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let seq = lines
            .next()
            .map(Sequence::from_str)
            .ok_or(NetworkError::GetSequence)??;

        let nodes = lines
            // Skip empty line between sequence and network
            .skip(1)
            .map(|s| {
                let mut split = s.split(" = ");

                let node = split.next().ok_or_else(|| NetworkError::ParseNode(s.to_string()))?;
                let node_addr = split.next().ok_or_else(|| NetworkError::ParseNodeAddr(s.to_string()))?;

                Ok((Node::from_str(node)?, NodeAddress::from_str(node_addr)?))
            })
            .collect::<Result<Vec<(Node, NodeAddress)>, NetworkError>>()?;

        Ok(Self {
            nodes: nodes.into_iter().collect(),
            seq,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_1_GHOST: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn example_1() {
        let network = Network::from_str(EXAMPLE_1).expect("Failed parsing network");
        let steps = network
            .find_steps_required_for(&Node::from_str("ZZZ").expect("Failed parsing node"))
            .expect("Failed getting steps");
        assert_eq!(steps, 2);
    }

    #[test]
    fn example_2() {
        let network = Network::from_str(EXAMPLE_2).expect("Failed parsing network");
        let steps = network
            .find_steps_required_for(&Node::from_str("ZZZ").expect("Failed parsing node"))
            .expect("Failed getting steps");
        assert_eq!(steps, 6);
    }

    #[test]
    fn example_1_ghost() {
        let network = Network::from_str(EXAMPLE_1_GHOST).expect("Failed parsing network");
        let steps = network
            .find_ghost_steps_required_for('Z')
            .expect("Failed getting steps");
        assert_eq!(steps, 6);
    }

    #[test]
    fn get_first_node() {
        let aaa_node = Node::from_str("AAA").expect("Failed parsing node");

        let network_1 = Network::from_str(EXAMPLE_1).expect("Failed parsing network");
        let first = network_1.nodes.keys().next().expect("Failed to get first node");

        assert_eq!(&aaa_node, first);

        let network_2 = Network::from_str(EXAMPLE_2).expect("Failed parsing network");
        let first = network_2.nodes.keys().next().expect("Failed to get first node");

        assert_eq!(&aaa_node, first);
    }
}
