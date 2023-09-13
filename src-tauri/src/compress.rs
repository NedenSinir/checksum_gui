use brotli::{CompressorWriter, enc::BrotliEncoderParams, CompressorReader};
use serde_derive::{Serialize, Deserialize};
use std::{io::{Write, self, BufRead}, fs::File, collections::HashMap};
use brotli::Decompressor;
use std::io::Read;

use crate::generate_enum_combinatorics::{PredefinedClass, IdentifierClass};


//use crate::{structs::hex::HexString};
    

pub fn compress_apc(input_file_path: &str, output_file_path: &str) -> std::io::Result<()> {
    // Open the input text file
    let input = File::open(input_file_path)?;
    // Create a Brotli encoder with default parameters
    let params = BrotliEncoderParams::default();
    let mut reader = CompressorReader::with_params(input, 4096, &params);

    // Create or truncate the output file
    let mut output = File::create(output_file_path)?;

    // Compress the file and write the output
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;


    output.write_all(&buffer)?;
    println!("{}","compress completed");
    Ok(())
}

pub fn decompress_apc(input_file_path: &str) -> std::io::Result<HashMap<[u8;2], PredefinedClass>> {
    // Open the input binary file
    println!("Decompress from apc started");

    let input = File::open(input_file_path)?;

    // Create a Brotli decompressor
    let mut decompressor = Decompressor::new(input, 4096);

    // Decompress the file
    let mut buffer = Vec::new();
    decompressor.read_to_end(&mut buffer)?;

    // Convert the buffer into text
    let text = String::from_utf8(buffer).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    // Split the text into chunks and insert them into the HashMap
    let mut hashmap: HashMap<[u8; 2], PredefinedClass> = HashMap::new();

    for line in text.lines() {
        if let Ok(interval_object) = serde_json::from_str::<PredefinedClass>(line) {
            let id_parts: Vec<&str> = interval_object.id.split(',').collect();
            if id_parts.len() == 2 {
                let id_array: [u8; 2] = [
                    id_parts[0].parse().unwrap(),
                    id_parts[1].parse().unwrap(),
                ];
                hashmap.insert(id_array, interval_object);
            }
        }
    }
    println!("Decompress from apc is completed");

    Ok(hashmap)
}

pub fn compress_layer_data(data:Vec<u8>)->Vec<u8>{

 // Create a Brotli compressor
 
 let mut compressor = CompressorWriter::new(Vec::new(), 4096 , 11,22);

 // Write the input data to the compressor
 match compressor.write_all(&data) {
     Ok(_) => (),
     Err(e) => panic!("Failed to write data to compressor: {:?}", e),
 }

 // Finalize the compressor to flush any remaining data
 match compressor.flush() {
     Ok(_) => (),
     Err(e) => panic!("Failed to flush compressor: {:?}", e),
 }

 // Get the compressed data from the compressor
 
 let compressed_data = compressor.into_inner();
 if compressed_data.len() > data.len() {
     return data
    }
    println!("{} <-original and {} <- compressed",data.len(),compressed_data.len());
 compressed_data   
}

pub fn read_identifiers_from_file(file_path: &str) -> io::Result<HashMap<[u8; 6], String>> {
    let mut identifier_map: HashMap<[u8; 6], String> = HashMap::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let identifier: IdentifierClass = serde_json::from_str(&line)?;

        identifier_map.insert(identifier.pattern, identifier.id);
    }

    Ok(identifier_map)
}