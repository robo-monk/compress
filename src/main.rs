mod huffman;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// input's filename
   #[clap(short, long, value_parser)]
   input: String,

   /// output's filename
   #[clap(short, long, value_parser, default_value = "")]
   output: String,


   /// decompress file
   #[clap(short, long, value_parser, default_value_t = false)]
   decompress: bool
}

fn write_bytes_to_file(filename: &str, bytes: &[u8]) -> std::io::Result<()> {
    {
        let mut file = File::create(filename)?;
        // Write a slice of bytes to the file
        file.write_all(bytes)?;
    }

    Ok(())
}

fn read_bytes_from_file(filename: &str) -> std::io::Result<Vec::<u8>> {

    let mut buffer = Vec::<u8>::new();

    {
        let mut file = File::open(filename)?;
        // read the same file back into a Vec of bytes
        file.read_to_end(&mut buffer)?;
        // println!("{:?}", buffer);
    }

    Ok(buffer)
}

fn compress(input_path: String, mut output_path: String) {
    if output_path.len() == 0 {
        output_path =  format!("{input_path}.zzz");
    }

    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let tree = huffman::Tree::new_from(String::from(input.to_owned()));
    let serialization = tree.serialize();

    // DEBUG
    let _node = huffman::decode(serialization.to_owned());
    assert!(serialization == _node.serialize());



    let mut cache: HashMap<char, Vec<bool>> = HashMap::new();

    let mut current_byte_index: usize = 0;
    let mut current_byte: u8 = 0;

    let mut bytes: Vec<u8> = serialization.as_bytes().to_vec();

    input.to_owned().chars().for_each(|c| {
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

    let mut serialization_bytes = Vec::new();
    write!(&mut serialization_bytes, "{serialization}").expect("could not write buffer");
    serialization_bytes.push(huffman::NEW_LINE_BYTE);
    let write_bytes: Vec<u8> = serialization_bytes.into_iter().chain(bytes.into_iter()).collect();
    let result = write_bytes_to_file(&output_path, &write_bytes).expect("could not write file");

    // println!("{}\n{}", serialization, compressed_debug.concat());
    let original_size = &input.len();
    let compressed_size = &write_bytes.len();
    println!("original size: {} bytes \n", original_size);
    println!("compressed size: {} bytes \n", compressed_size);
    let compression_rate: f32 = (100 * compressed_size / original_size) as f32;
    println!("-> {}% compressed", 100.0 - compression_rate);
    println!("-> written to {}", output_path);

}

fn decompress(input_path: String, output_path: String) {
    let output_zip = fs::read(input_path).expect("Should have been able to read the file");
    let group_seperator_index = output_zip.to_vec().into_iter().position(|byte| byte == huffman::NEW_LINE_BYTE).unwrap() + 1; 

    let mut o = output_zip.to_owned();
    let serilization_input: Vec<u8> = o.splice(0..group_seperator_index, None).collect();
    println!("> serialization len {}", serilization_input.len());
    let output_tree2 = huffman::Tree::new_from_serialization_bytes(&serilization_input);
    // println!("> {}", output_tree.serialize());
    println!("> {}", output_tree2.serialize());

    let tape: Vec<u8> = o.splice((group_seperator_index-1).., None).collect();


    let mut bits: Vec<bool> = Vec::new();
    tape.into_iter().for_each(|mut byte| {
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

    println!("done reading bits");

    let mut values: Vec<huffman::Val> = Vec::new();
    let mut current_node = output_tree2.root.to_owned();

    bits.into_iter().for_each(|bit| {
        let mut node = current_node.to_owned();

        if node.is_leaf() {
            let value: huffman::Val = node.value;
            values.push(value);

            node = output_tree2.root.to_owned();
        }

        if bit {
            let right = *node.right.unwrap();
            current_node = right;
            // current_node = current_node.right.to.as_ref().unwrap();
        } else {
            let left = *node.left.unwrap();
            current_node = left;
        }
    });


    println!("done traversing tree");
    let actual_values: Vec<String> = values.into_iter().map(|value| value.unwrap()).collect();
    let text = actual_values.join("");
    println!("text is {text}");


    let mut write_bytes = Vec::new();
    write!(&mut write_bytes, "{text}").expect("could not write buffer");
    let result = write_bytes_to_file(&output_path, &write_bytes).expect("could not write file");

    // println!("> {}", tape);
}

fn main() {
    let args = Args::parse();

    println!("input {}!", args.input);
    println!("output {}!", args.output);
    println!("decompress? {}!", if args.decompress { "true" } else { "false" });

    let input_path = args.input;
    let output_path = args.output;

    let action = if args.decompress { decompress } else { compress };

    action(input_path, output_path);


    // serilization_input.to_owned().into_iter().for_each(|b| {
    //     let c  = char::from(b);
    //     print!("{c}|");
    // });

    // let chars_ser: Vec<String> = serilization_input.to_owned().into_iter().map(|b| {
    //     let c  = char::from(b);
    //     // print!("{c}|");
    //     format!("{c}")
    // }).collect();

    // let ser = chars_ser.join("");
    // println!("\n--");
    
    // let output_tree = huffman::Tree::new_from_bytes(&serilization_input);
    // let output_tree = huffman::Tree::new_from(ser);
   // println!("> {}", serilization_input);
    // output_zip.to_vec().into_iter().enumerate().for_each(|(index, byte)| {
    //     let char = char::from(byte);
    //     if char == huffman::NEW_LINE {
    //         println!("> {index} <{char}>");
    //     }
    // });

    // let output = fs::read_to_string(format!("./{output_path}")).expect("Should have been able to read the file");
    // let output = fs::read_to_string("./test.txt.zzz").unwrap();
// 
    // println!("-> output {}", format!("./{output}"));


    // let read_bytes = read_bytes_from_file(filename).expect("error reading file");
    // // println!("read_bytes: {} \n", read_bytes.len());

    // let mut bits: Vec<bool> = Vec::new();
    // read_bytes.into_iter().for_each(|mut byte| {
    //     byte = byte.reverse_bits();
    //     for _ in 0..8 { // change it to get range
    //         let lsb = byte & 1;
    //         byte >>= 1;
    //         bits.push(if lsb == 0 { false } else { true });
    //     }

    //     // println!("le bytes {bit} || {bit3}");
    //     // let bit = byte << 1;
    //     // let bit3 = byte.rotate_left(1);
    //     // // let le_bytes = byte.to_le_bytes().map(|b| b.to_string());
    //     // // println!("le bytes {} || {}", le_bytes.join(" "))
    //     // println!("le bytes {bit} || {bit3}")
    // });

    // println!("done reading bits");

    // let mut values: Vec<huffman::Val> = Vec::new();
    // let mut current_node = tree.to_owned().root;

    // // let mut _current_path: Vec<bool> = Vec::new();
    // // let mut _cache: HashMap<Vec<bool>, huffman::Val> = HashMap::new();
    // // very very slow!
    // bits.into_iter().for_each(|bit| {
    //     let mut node = current_node.to_owned();

    //     if node.is_leaf() {
    //         let value: huffman::Val = node.value;
    //         values.push(value);
    //         // current_node = tree.root.to_owned();
    //         node = tree.to_owned().root;
    //     }

    //     if bit {
    //         let right = *node.right.unwrap();
    //         current_node = right;
    //         // current_node = current_node.right.to.as_ref().unwrap();
    //     } else {
    //         let left = *node.left.unwrap();
    //         current_node = left;
    //     }
    // });

    // println!("done traversing tree");
    // let actual_values: Vec<String> = values.into_iter().map(|value| value.unwrap()).collect();
    // let text = actual_values.join("");
    // println!("text is {text}");
}
