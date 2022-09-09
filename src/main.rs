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

fn compress(input_path: String, mut output_path: String) {
    if output_path.len() == 0 {
        output_path =  format!("{input_path}.zzz");
    }

    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let tree = huffman::Tree::new_from(String::from(input.to_owned()));
    let serialization = tree.serialize();

    // DEBUG
    // let _node = huffman::decode(serialization.to_owned());
    // assert!(serialization == _node.serialize());


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


    let mut values: Vec<huffman::Val> = Vec::new();
    let mut current_node = output_tree2.root.to_owned();

    tape.into_iter().for_each(|mut byte| {
        byte = byte.reverse_bits();
        for _ in 0..8 { // change it to get range
            let lsb = byte & 1;
            byte >>= 1;

            let mut node = current_node.to_owned();

            if node.is_leaf() {
                let value: huffman::Val = node.value;
                values.push(value);

                node = output_tree2.root.to_owned();
            }

            if lsb == 1 {
                let right = *node.right.unwrap();
                current_node = right;
            } else {
                let left = *node.left.unwrap();
                current_node = left;
            }
        }
    });

    println!("done traversing tree");
    let actual_values: Vec<String> = values.into_iter().map(|value| value.unwrap()).collect();
    let text = actual_values.join("");
    println!("text len is {}", text.len());


    let mut write_bytes = Vec::new();
    write!(&mut write_bytes, "{text}").expect("could not write buffer");
    write_bytes_to_file(&output_path, &write_bytes).expect("could not write file");
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
}
