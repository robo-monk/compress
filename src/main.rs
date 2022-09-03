use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;


const L_BRACKET: char = '\u{008E}';
const R_BRACKET: char = '\u{008F}';
const COMMA: char = '\u{0090}';

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

type Freq = u32;
type Val = Option<String>;

#[derive(Clone)]
struct Node {
    frequency: Freq,
    value: Val,

    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
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

    pub fn unbox(&self) -> &Node {
        self
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

    fn traverse(&self, value: Val) -> Option<Vec<bool>> {
        self.root.traverse(&value, &vec![])
    }
    // or traverse as name
    // fn compile(&self) -> HashMap<u32, Val> {
    //     let map: HashMap<u32, Val> = HashMap::new();
    //     map.insert(0, );
    //     map
    // }
}

fn write_bytes_to_file(filename: &str, bytes: &[u8]) -> std::io::Result<()> {
    {
        let mut file = File::create(filename)?;
        // Write a slice of bytes to the file
        file.write_all(bytes)?;
    }

    // {
    //     let mut file = File::open(filename)?;
    //     // read the same file back into a Vec of bytes
    //     let mut buffer = Vec::<u8>::new();
    //     file.read_to_end(&mut buffer)?;
    //     println!("{:?}", buffer);
    // }
    Ok(())
}

fn read_bytes_from_file(filename: &str) -> std::io::Result<Vec::<u8>> {

    let mut buffer = Vec::<u8>::new();

    {
        let mut file = File::open(filename)?;
        // read the same file back into a Vec of bytes
        file.read_to_end(&mut buffer)?;
        println!("{:?}", buffer);
    }

    Ok(buffer)
}


// fn convert_bit_buffer_to_bytes(buf: Vec::<bool>) -> Vec::<u8> {
//     //  iter.reduce(|accum, item| {
//     //     if accum >= item { accum } else { item }
//     // })
//     let mut current_byte: u8 = 0;
//     let mut bytes_buffer: Vec::<u8> = Vec::new();

//     buf.into_iter().reduce(|prev, bit| {
//         println!("{:?}", prev);
//         // current_byte.rotate_left
//         return false
//     });

//     return bytes_buffer;
// }

fn main() {
    let file_path = "./test.txt";
    println!("In file {}", file_path);

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{input}");

    let tree = Tree::new_from(String::from(input.to_owned()));
    let serialization = tree.serialize();
    println!("{}", serialization);

    let _node = decode(serialization.to_owned());

    println!("encoded> {}", serialization);
    println!("decoded> {}", _node.serialize());
    println!(
        "error in serialization? {}",
        if serialization == _node.serialize() {
            " no "
        } else {
            "yes "
        }
    );

    // println!("string for 2> {}", serialized_code.concat());

    // input.to_owned().chars().for_each(|c| {
    //     let mut code = tree.traverse(Some(c.to_string())).unwrap_or(vec![]);
    //     let mut serialized_code: Vec<&str> = code
    //                             .into_iter()
    //                             .map(|c| if c { "1" } else { "0" })
    //                             .collect();

    //     println!("{} -> {}", c, serialized_code.concat());
    // });
    let mut cache: HashMap<char, Vec<bool>> = HashMap::new();

    let compressed_debug: Vec<String> = input
        .to_owned()
        .chars()
        .map(|c| {
            // let mut code;
            if !cache.contains_key(&c) {
                cache.insert(c, tree.traverse(Some(c.to_string())).unwrap_or(vec![]));
            }
            let code = cache.get(&c).unwrap();

            let serialized_code: Vec<&str> = code
                .into_iter()
                .map(|&c| if c { "1" } else { "0" })
                .collect();
            // code
            // .into_iter()
            // .map(|&c| if c { "1" } else { "0" })

            // let serialized_bytes: Vec<u8> =
            //                         .collect();

            return serialized_code.concat();
        })
        .collect();

    let mut current_byte_index: usize = 0;
    let mut current_byte: u8 = 0;
    let mut bytes: Vec<u8> = Vec::new();

    input.to_owned().chars().enumerate().for_each(|(i, c)| {
        if !cache.contains_key(&c) {
            cache.insert(c, tree.traverse(Some(c.to_string())).unwrap_or(vec![]));
        }

        let bits = cache.get(&c).unwrap();
        bits.iter().for_each(|bit| {
            if current_byte_index != 0 && current_byte_index % 8 == 0 {
                bytes.push(current_byte);
                current_byte = 0;
            }

            current_byte <<= 1;
            current_byte += if *bit { 1 } else { 0 };
            current_byte_index += 1;
        });
    });

    // this has a bug, we'll also need to shift these the remaining of the byte positions
    if current_byte > 0 {
        bytes.push(current_byte)
    } // add remaining bits if existent

    println!("bytes len {}", bytes.len());
    bytes.to_owned().into_iter().for_each(|c| {
        println!(">{:#02b}", c);
        // String::from("a")
    });

    // let mut file = File::create("test.txt.zzz");
    let filename = "test.txt.zzz";

    let result = write_bytes_to_file(filename, &bytes);


    println!("{}\n{}", serialization, compressed_debug.concat());
    let original_size = input.len() * 8;
    let compressed_size = compressed_debug.concat().len() + serialization.len();
    println!("original size: {} bits \n", original_size);
    println!("compressed size: {} bits \n", compressed_size);
    let compression_rate: f32 = (100 * compressed_size / original_size) as f32;
    println!("-> {}% compressed", 100.0 - compression_rate);


    /// 
    let read_bytes = read_bytes_from_file(filename).expect("error reading file");
    println!("read_bytes: {} \n", read_bytes.len());

    let mut bits: Vec<bool> = Vec::new();
    read_bytes.into_iter().for_each(|mut byte| {
        byte = byte.reverse_bits();


        for _ in 0..8 { // change it to get range
            let lsb = byte & 1;
            byte >>= 1;
            bits.push(if lsb == 0 { false } else { true });
        }

        // println!("le bytes {bit} || {bit3}");
        // let bit = byte << 1;
        // let bit3 = byte.rotate_left(1);
        // // let le_bytes = byte.to_le_bytes().map(|b| b.to_string());
        // // println!("le bytes {} || {}", le_bytes.join(" "))
        // println!("le bytes {bit} || {bit3}")
    });

    let mut values: Vec<Val> = Vec::new();
    let mut current_node = tree.to_owned().root;

    bits.into_iter().for_each(|bit| {
        if current_node.to_owned().is_leaf() {
            let value: Val = current_node.to_owned().value;
            values.push(value);
            // current_node = tree.root.to_owned();
            current_node = tree.to_owned().root;
        }

        if bit {
            let right = *current_node.to_owned().right.unwrap();
            current_node = right;
            // current_node = current_node.right.to.as_ref().unwrap();
        } else {
            let left = *current_node.to_owned().left.unwrap();
            current_node = left;
        }
    });

    let actual_values: Vec<String> = values.into_iter().map(|value| value.unwrap()).collect();
    let text = actual_values.join("");
    println!("text is {text}");

    // // let mut code;
    // if !cache.contains_key(&c) {
    //     cache.insert(c, tree.traverse(Some(c.to_string())).unwrap_or(vec![]));
    // }
    // let code = cache.get(&c).unwrap();

    // let serialized_code: Vec<&str> = code
    //                         .into_iter()
    //                         .map(|&c| if c { "1" } else { "0" })
    //                         .collect();
    // // code
    //     // .into_iter()
    //     // .map(|&c| if c { "1" } else { "0" })

    // // let serialized_bytes: Vec<u8> =
    // //                         .collect();

    // return serialized_code.concat()
    // });

    // println!("-> bytes \n\n {} \n\n", serialized_bytes.join(" "));
}

fn decode(serialization: String) -> Node {
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
