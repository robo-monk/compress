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
            self.left.as_ref().unwrap().serialize(),
            self.right.as_ref().unwrap().serialize()
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
    // Node::new(8, String::from("test"));
}

fn decode(serialization: String) -> Node {
    let mut chars = serialization.chars();
    let mut map: HashMap<char, u32> = HashMap::new();

    let mut bracket_counter: i32 = -1;
    let mut starter_bracket_index: i32 = -1;

    let mut buffer: Vec<char> = Vec::new();

    let mut parent_node = Node::new(0, None);

    let cursor = chars.next();
    if cursor.is_none() { return Node::new(0, None) }

    match cursor.unwrap() {
        '[' => {
            println!("<")
        },
        ']' => {
            println!(">")
        },
        _ => {

            println!("~")
        }
    }

    buffer.push(cursor.unwrap());

    // for (i, c) in chars.enumerate() {

    //     buffer.push(c);
    //     if c == '[' {
    //         bracket_counter += 1;
    //         if bracket_counter == 0 {
    //             starter_bracket_index = i as i32;
    //             bracket_counter = 0;
    //         }
    //     }

    //     if c == ']' {
    //         // println!("] {}", bracket_counter);
    //         bracket_counter -= 1;
    //         if bracket_counter == -1 {
    //             buffer = buffer.drain(1..(buffer.len()-1)).collect(); // remove first [

    //             let s = String::from_iter(&buffer);
    //             // let serialization = decode((&s).to_string());
    //             println!("#{} branch {}", starter_bracket_index, s);

    //             let mut buffer_split = s.split(',').into_iter();
    //             let serialized_value = buffer_split.next().unwrap();
    //             let frequency = buffer_split.next().unwrap();
    //             // let value = serialized_value.get(0);

    //             let parsed_frequency: u32 = frequency.parse().unwrap();
    //             let parsed_value: char = serialized_value.chars().next().unwrap();
    //             // let parsed_value: u32 = serialized_value.parse().unwrap();

    //             if starter_bracket_index == 0 {
    //                 parent_node = Node::new((parsed_frequency, parsed_value));
    //                 // parent_node.frequency = frequency;
    //             }

    //             println!("~~> parse freq: {}", parsed_frequency);
    //             println!("~~> parse value: {}", parsed_value);
    //             // println!("~~> parse frequ: {}", parsed_value);
            
    //             // buffer_split.map(|chunk| chunk + ",");
    //             // let branches = String::from_iter(buffer_split);
    //             let branches = String::from_iter(buffer_split.map(|chunk| format!("{},", chunk)));
                
    //             // println!(">> blen {}", branches.len());
    //             if branches.len() == 0 {
    //                 // let _frequency: u32 = frequency.to_owned().into();
    //                 // let node = Node::new((parsed_frequency, parsed_value));
    //                 // node.serialize_debug();
    //                 // println!("")
    //             } else {
    //                 println!("---- more branches ----");
    //                 let mut node = decode(branches.to_owned());
    //                 // parent_node.left = node;
    //                 // parent_node = Node::new((parent_node.frequency(), parent_node.value()));
    //                 let mut right: Option<Box<Node<(u32, char)>>> = None;

    //                 if node.left.is_some() { right = node.left.unwrap().into() };
    //                 let left: Option<Box<Node<(u32, char)>>> = Node::new((node.v.0.to_owned(), node.v.1.to_owned())).into();
    //                 // let right: Option<Box<Node<(u32, char)>>> = node.left().unwrap().into();

    //                 parent_node = Node {
    //                     v: (parent_node.frequency(), parent_node.value()),
    //                     left,
    //                     right
    //                 };

    //                 println!("debug node >");
    //                 println!("debug node left>");
    //                 println!("-----");
    //             }

    //             bracket_counter = -1;
    //             buffer.clear()
    //         }
    //     }
    // }

    return parent_node;
}
