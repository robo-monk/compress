use std::collections::HashMap;

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

type Freq = u8;
type Val = Option<String>;

struct Node {
    frequency: Freq,
    value: Val,

    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
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
        // let serialized_value = self.value().to_digit(10).unwrap_or(0);
        let empty_value = &String::from("");
        let serialized_value = self.value.as_ref().unwrap_or(empty_value);

        if self.is_leaf() {
            return format!("[{},{}]", serialized_value, self.frequency);
        }


        return format!(
            "[{},{},{},{}]",
            serialized_value,
            self.frequency,
            self.left.as_ref().unwrap_or(&Box::from(Node::new(0, None))).serialize(),
            self.right.as_ref().unwrap_or(&Box::from(Node::new(0, None))).serialize()
        );
    }
}

struct Tree {
    root: Node,
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

    fn new_from(s: String) -> Self {
        let frequency_map = get_frequency_map(s.to_owned());
        let queue = frequency_map_to_queue(frequency_map);
        queue_to_tree(queue)
    }

    // fn new_from_serialization(serialization: String) -> Self {
    //     let chars
    // }

    fn serialize(&self) -> String {
        self.root.serialize()
    }
}

fn main() {
    let tree = Tree::new_from(String::from("gamw2222222222"));
    let serialization = tree.serialize();
    println!("{}", serialization);

    let _node = decode(serialization.to_owned());

    println!("encoded> {}", serialization);
    println!("decoded> {}", _node.serialize());
    // Node::new(8, String::from("test"));
}

fn decode(serialization: String) -> Node {
    let mut chars = serialization.chars();
    let mut map: HashMap<char, u32> = HashMap::new();

    let mut bracket_counter: i32 = 0;
    let mut starter_bracket_index: i32 = -1;

    let mut buffer: Vec<char> = Vec::new();

    let mut index = 0;
    let mut parent_node = Node::new(0, None);

    while true {
        let cursor = chars.next();
        if cursor.is_none() {
            break;
        };

        let c = cursor.unwrap();

        match c {
            '[' => {
                if bracket_counter > 0 {
                    buffer.push(c);
                }

                // starter_bracket_index = index;
                bracket_counter += 1;
            }
            ']' => {
                bracket_counter -= 1;
                buffer.push(c);
                // println!("branch found start: {}, end: {}", starter_bracket_index, index);
                // println!("]");
                if bracket_counter == 0 {
                    let s = String::from_iter(&buffer);
                    println!(">> {}", s);

                    // decode(s);
                    buffer.clear();
                } else if bracket_counter == 1 {
                    let branch = String::from_iter(&buffer);
                    if parent_node.left.is_none() {
                        let node = decode(branch.to_owned());
                        println!(">> node > {}", node.serialize());
                        parent_node.left = decode(branch.to_owned()).into();
                        println!(">> found left branch > {}  \n\n {} \n\n", branch, parent_node.serialize());
                    } else if parent_node.right.is_none() {
                        parent_node.right = decode(branch.to_owned()).into();
                        println!(">> found left branch > {}", branch);
                    } else {
                        println!(">> invalid serialization");
                    }

                    buffer.clear();
                }
            }
            ',' => {
                if parent_node.value.is_none() {
                    println!("~ value > {}", String::from_iter(&buffer));
                    parent_node.value = Some(String::from_iter(&buffer));
                    buffer.clear();
                } else if parent_node.frequency == 0 {
                    parent_node.frequency = String::from_iter(&buffer).parse().unwrap_or(255);
                    println!("~ freq > {}", parent_node.frequency);
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

        index += 1;
    }

    return parent_node;
}
