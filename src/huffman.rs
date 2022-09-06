use std::collections::HashMap;
pub const L_BRACKET: char = '\u{008E}';
pub const R_BRACKET: char = '\u{008F}';
pub const COMMA: char = '\u{0090}';
pub const NEW_LINE_BYTE: u8 = 29;
pub const NEW_LINE: char = NEW_LINE_BYTE as char;

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


// fn render_char(Sp)

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
        // let serialized_value = self.value().to_digit(10).unwrap_or(0);
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
            // let mut value;
            // let code = code;
            let mut traversed_code = None;

            // println!("lef is noe ? {}", self.left.is_none());

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
            // return Some(traversed_code)
        }

        // println!("self value {} {}", self.value.as_ref().unwrap(), value.to_owned().unwrap());
        if self.value != *value {
            return None;
        }
        // println!("found it! {}", code.len());

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
    // let mut queue: Vec<(u32, Node<(u32, char)>)> = Vec::new();
    let mut queue: Vec<Node> = Vec::new();
    // let mut queue: Vec<(char, u32)> = map.values().into();
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

impl Tree {
    fn new(root: Node) -> Self {
        Tree { root }
    }

    pub fn new_from(s: String) -> Self {
        let frequency_map = get_frequency_map(s.to_owned());
        let queue = frequency_map_to_queue(frequency_map);
        queue_to_tree(queue)
    }

    // fn new_from_serialization(serialization: String) -> Self {
    //     let chars
    // }

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
                    let s = String::from_iter(&buffer);
                    // println!("clear buffer >> {}", s);
                    // println!("parent node is >> {}", parent_node.serialize());
                    // println!("----");
                    // decode(s);
                    buffer.clear();
                } else if bracket_counter == 1 {
                    let branch = String::from_iter(&buffer);
                    // println!("parent node is >> {}", parent_node.serialize());
                    if parent_node.left.is_none() {
                        // let node = decode(branch.to_owned());
                        // println!(">> node > {}", node.serialize());
                        // println!("left branch >> {}", branch);
                        parent_node.left = decode(branch.to_owned()).into();
                        // println!(">> found left branch > {}  \n\n {} \n\n", branch, parent_node.serialize());
                    } else if parent_node.right.is_none() {
                        // println!("right branch >> {}", branch);
                        let mut trimmed_branch = branch.to_owned();
                        trimmed_branch.remove(0);
                        parent_node.right = decode(trimmed_branch).into();
                        // println!(">> found right branch > {}", branch);
                    } else {
                        println!(">> invalid serialization");
                    }

                    buffer.clear();
                }
            }
            COMMA => {
                if parent_node.value.is_none() {
                    // println!("~ value > {}", String::from_iter(&buffer));
                    parent_node.value = Some(String::from_iter(&buffer));
                    buffer.clear();
                } else if parent_node.frequency == 0 {
                    // println!("~ freq > {}", String::from_iter(&buffer));
                    parent_node.frequency = String::from_iter(&buffer).parse().unwrap_or(255);
                    // println!("~ freq > {}", parent_node.frequency);
                    buffer.clear();
                } else {
                    buffer.push(c);
                }
            }
            _ => {
                buffer.push(c);
                // println!("~ {}", c);
            }
        }

        // index += 1;
    }

    return parent_node;
}
