mod huffman;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

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

    let tree = huffman::Tree::new_from(String::from(input.to_owned()));
    let serialization = tree.serialize();
    println!("{}", serialization);

    let _node = huffman::decode(serialization.to_owned());

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

    let mut values: Vec<huffman::Val> = Vec::new();
    let mut current_node = tree.to_owned().root;

    bits.into_iter().for_each(|bit| {
        if current_node.to_owned().is_leaf() {
            let value: huffman::Val = current_node.to_owned().value;
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
