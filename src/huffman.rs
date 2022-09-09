use std::collections::HashMap;

pub const NEW_LINE_BYTE: u8 = 29;
pub const NEW_LINE: char = NEW_LINE_BYTE as char;

pub const L_BRACKET_BYTE: u8 = 15; // ASCII esc. code for "shift in"
pub const R_BRACKET_BYTE: u8 = 14; // ASCII esc. code for "shift out"
pub const COMMA_BYTE: u8 = 26; // ASCII esc. code for "substitue" !WILL REMOVE CAUSE ITS NOT NECESARY


pub const L_BRACKET: char = L_BRACKET_BYTE as char;
pub const R_BRACKET: char = R_BRACKET_BYTE as char;
pub const COMMA: char = COMMA_BYTE as char;


impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

pub type Freq = u32;
pub type Val = Option<String>;

#[derive(Clone)]
pub struct Node {
    pub frequency: Freq,
    pub value: Val,

    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    fn new(frequency: Freq, value: Val) -> Self {
        Node {
            frequency,
            value,
            left: None,
            right: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.right.is_none() && self.left.is_none()
    }

    pub fn serialize(&self) -> String {
        let empty_value = &String::from("");
        let serialized_value = self.value.as_ref().unwrap_or(empty_value);

        if self.is_leaf() {
            return format!(
                "{L_BRACKET}{}{COMMA}{}{COMMA}{R_BRACKET}",
                serialized_value, self.frequency
            );
        }

        return format!(
            "{L_BRACKET}{}{COMMA}{}{COMMA}{}{COMMA}{}{R_BRACKET}",
            serialized_value,
            self.frequency,
            self.left
                .as_ref()
                .unwrap_or(&Box::from(Node::new(0, None)))
                .serialize(),
            self.right
                .as_ref()
                .unwrap_or(&Box::from(Node::new(0, None)))
                .serialize()
        );
    }

    pub fn traverse(&self, value: &Val, mut code: &Vec<bool>) -> Option<Vec<bool>> {
        if !self.is_leaf() {
            let mut traversed_code = None;

            if self.left.is_some() {
                let mut new_code = code.clone();
                new_code.push(false);
                traversed_code = self.left.as_ref().unwrap().traverse(&value, &new_code);
            }

            if traversed_code.is_none() && self.right.is_some() {
                let mut new_code = code.clone();
                new_code.push(true);
                traversed_code = self.right.as_ref().unwrap().traverse(&value, &new_code)
            }

            return traversed_code;
        }

        if self.value != *value {
            return None;
        }

        return Some(code.to_owned());
    }
}


#[derive(Clone)]
pub struct Tree {
    pub root: Node,
}

fn queue_to_tree(mut queue: Vec<Node>) -> Tree {
    if queue.len() == 1 {
        return Tree::new(queue.pop().unwrap());
    }

    queue.sort_by(|a, b| b.frequency.cmp(&a.frequency));

    let left = queue.pop().unwrap();
    let right = queue.pop().unwrap();
    let combined_frequency = left.frequency + right.frequency;
    let new_node = Node {
        frequency: combined_frequency,
        value: None,
        right: right.into(),
        left: left.into(),
    };

    queue.push(new_node);

    return queue_to_tree(queue);
}

// converts a HashMap of (char, frequency) to a Vec of (freq, Node)
fn frequency_map_to_queue(map: HashMap<char, Freq>) -> Vec<Node> {
    let mut queue: Vec<Node> = Vec::new();
    for (c, frequency) in map.iter() {
        println!("char: {c} freq: {frequency}");
        queue.push(Node::new(*frequency, Some(c.to_string())))
    }

    queue
}

fn get_frequency_map(s: String) -> HashMap<char, Freq> {
    let chars = s.chars();

    let mut frequency_map: HashMap<char, Freq> = HashMap::new();

    for c in chars {
        let mut char_frequency: Freq = 0;

        if frequency_map.contains_key(&c) {
            char_frequency = *frequency_map.get(&c).unwrap();
        }

        frequency_map.insert(c, char_frequency + 1);
    }

    frequency_map
}

fn get_frequency_map_from_bytes(bytes: &Vec<u8>) -> HashMap<char, Freq> {
    // let chars = s.chars();

    let mut frequency_map: HashMap<char, Freq> = HashMap::new();
    for b in bytes {
        let c = char::from(*b);
        print!("{c}");
        let mut char_frequency: Freq = 0;

        if frequency_map.contains_key(&c) {
            char_frequency = *frequency_map.get(&c).unwrap();
        }

        frequency_map.insert(c, char_frequency + 1);
    }

    frequency_map
}

impl Tree {
    fn new(root: Node) -> Self {
        Tree { root }
    }

    pub fn new_from(s: String) -> Self {
        let frequency_map = get_frequency_map(s.to_owned());
        let queue = frequency_map_to_queue(frequency_map);
        queue_to_tree(queue)
    }

    pub fn new_from_bytes(bytes: &Vec<u8>) -> Self {
        let frequency_map = get_frequency_map_from_bytes(bytes);
        let queue = frequency_map_to_queue(frequency_map);
        queue_to_tree(queue)
    }

    pub fn new_from_serialization(serialization: String) -> Self {
        let root = decode(serialization);
        Tree { root }
    }
    pub fn new_from_serialization_bytes(bytes: &Vec<u8>) -> Self {
        let root = decode_bytes(bytes);
        Tree { root }
    }

    pub fn serialize(&self) -> String {
        self.root.serialize()
    }

    pub fn traverse(&self, value: Val) -> Option<Vec<bool>> {
        self.root.traverse(&value, &vec![])
    }
    // or traverse as name
    // fn compile(&self) -> HashMap<u32, Val> {
    //     let map: HashMap<u32, Val> = HashMap::new();
    //     map.insert(0, );
    //     map
    // }
}

pub fn decode_bytes(bytes: &Vec<u8>) -> Node {
    // let mut chars = serialization.chars();

    let mut bracket_counter: i32 = 0;

    // let mut buffer: Vec<char> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();

    let mut parent_node = Node::new(0, None);

    let mut bytes_iter = bytes.iter();
    while true {
        let cursor = bytes_iter.next();
        if cursor.is_none() { break };

        let b = cursor.unwrap();
        let c = char::from(*b);

        match b {
            &L_BRACKET_BYTE => {
                if bracket_counter > 0 {
                    buffer.push(*b);
                }

                bracket_counter += 1;
            }

            &R_BRACKET_BYTE => {
                bracket_counter -= 1;
                buffer.push(*b);

                if bracket_counter == 0 {
                    buffer.clear();
                } else if bracket_counter == 1 {
                    if parent_node.left.is_none() {
                        parent_node.left = decode_bytes(&buffer).into();
                    } else if parent_node.right.is_none() {
                        // NEED TO OPTIMIZE
                        let mut trimmed_branch = buffer.to_owned();
                        trimmed_branch.remove(0);
                        parent_node.right = decode_bytes(&trimmed_branch).into();
                    } else {
                        println!(">> invalid serialization");
                    }

                    buffer.clear();
                }
            }
            &COMMA_BYTE => {
                if parent_node.value.is_none() {
                    let chars = buffer.iter().map(|b| {
                        char::from(*b)
                    });

                    parent_node.value = Some(String::from_iter(chars));
                    buffer.clear();
                } else if parent_node.frequency == 0 {
                    let chars = buffer.iter().map(|b| {
                        char::from(*b)
                    });
                    parent_node.frequency = String::from_iter(chars).parse().unwrap_or(255);
                    
                    buffer.clear();
                } else {
                    buffer.push(*b);
                }
            }
            _ => {
                buffer.push(*b);
            }
        }

    }

    return parent_node;
}

pub fn decode(serialization: String) -> Node {
    let mut chars = serialization.chars();

    let mut bracket_counter: i32 = 0;

    let mut buffer: Vec<char> = Vec::new();

    let mut parent_node = Node::new(0, None);

    while true {
        let cursor = chars.next();
        if cursor.is_none() {
            break;
        };

        let c = cursor.unwrap();

        match c {
            L_BRACKET => {
                if bracket_counter > 0 {
                    buffer.push(c);
                }

                bracket_counter += 1;
            }

            R_BRACKET => {
                bracket_counter -= 1;
                buffer.push(c);

                if bracket_counter == 0 {
                    buffer.clear();
                } else if bracket_counter == 1 {
                    let branch = String::from_iter(&buffer);
                    if parent_node.left.is_none() {
                        parent_node.left = decode(branch.to_owned()).into();
                    } else if parent_node.right.is_none() {
                        let mut trimmed_branch = branch.to_owned();
                        trimmed_branch.remove(0);
                        parent_node.right = decode(trimmed_branch).into();
                    } else {
                        println!(">> invalid serialization");
                    }

                    buffer.clear();
                }
            }
            COMMA => {
                if parent_node.value.is_none() {
                    parent_node.value = Some(String::from_iter(&buffer));
                    buffer.clear();
                } else if parent_node.frequency == 0 {
                    parent_node.frequency = String::from_iter(&buffer).parse().unwrap_or(255);
                    buffer.clear();
                } else {
                    buffer.push(c);
                }
            }
            _ => {
                buffer.push(c);
            }
        }
    }

    return parent_node;
}
