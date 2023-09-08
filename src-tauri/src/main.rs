// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::{File, self}, vec, time::Instant, sync::{Arc, Mutex,RwLock}};
use std::io::{self, Write};

use app::{compress::{compress_apc, decompress_apc, compress_layer_data}, generate_classes_new::generate, primitivize::{primitivize_layer, LayerData}, revert::revert_layer};
use rayon::{prelude::{IntoParallelRefIterator, ParallelIterator}, slice::ParallelSlice};
use tauri::{Env, Context, Assets};
use ring::digest;
use rayon::iter::IndexedParallelIterator;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_apc,
            compress_to_apc,
            decompress_from_apc,
            primitivize_to_alpr,
            demo,
            revert_from_alpr
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn generate_apc() -> String {
    let a = generate();
    match a {
        Ok(()) => format!("good"),
        Err(e) => {
            format!("nogood")
        }
    }
}

#[tauri::command]
fn compress_to_apc() -> String {
  println!("{}","started");
  
    let a = compress_apc("../resources/output.txt", "../resources/data.apc");
    match a {
        Ok(()) => format!("good"),
        Err(e) => {
            format!("nogood")
        }
    }
}

#[tauri::command]
fn decompress_from_apc() {
    let a = decompress_apc("../resources/data.apc");
    match a {
        Ok((x)) => println!("{:?}",x),
        Err(e) => {
            format!("nogood");
        }
    }
}

#[tauri::command]
fn demo() {

    fn linear_interpolation(data: &str, new_length: usize) -> String {
        let data_len = data.len();
        
        // Check if the new length is less than or equal to the current length
        if new_length <= data_len {
            return data[..new_length].to_string(); // Truncate to the desired length
        }
        
        // Initialize an empty string for the result
        let mut result = String::new();
        
        // Calculate the step size for interpolation
        let step = (data_len - 1) as f64 / (new_length - 1) as f64;
        
        // Perform linear interpolation
        for i in 0..new_length {
            let index = (i as f64 * step).round() as usize;
            let interpolated_char = data.chars().nth(index).unwrap();
            result.push(interpolated_char);
        }
        
        result
    }

    fn reverse_linear_interpolation(data: &str, original_length: usize) -> String {
        let data_len = data.len();
        
        // Check if the original length is greater than or equal to the current length
        if original_length >= data_len {
            return data.to_string(); // Return the data as is
        }
        
        // Initialize an empty string for the result
        let mut result = String::new();
        
        // Calculate the step size for reverse interpolation
        let step = (data_len - 1) as f64 / (original_length - 1) as f64;
        
        // Perform reverse linear interpolation
        for i in 0..original_length {
            let index = (i as f64 * step).round() as usize;
            let char_at_index = data.chars().nth(index).unwrap();
            result.push(char_at_index);
        }
        
        result
    }

    let original_data = "010101010100101111001010";
    let new_length = 16;
    let interpolated_data = linear_interpolation(original_data, new_length);
    
    println!("{}", interpolated_data);

    let original_length = 24;
    let reversed_data = reverse_linear_interpolation(&interpolated_data, original_length);
    
    println!("{}", reversed_data);

    
    
}




#[tauri::command]
fn primitivize_to_alpr(file_path: &str) {
    

    // Open the file for reading
    
    // Read the contents of the file into a Vec<u8>
    let original_bytes = fs::read_to_string(file_path).unwrap().as_bytes().to_vec();
    let compressed_original_bytes = original_bytes;//compress_layer_data(original_bytes);
    println!("{:?} <-- itself",compressed_original_bytes);

    let universal_validator:Vec<u8> = digest::digest(&digest::SHA256, &compressed_original_bytes).as_ref().to_vec();
    let mut layer_amount:u32 = 0;
    let mut current_layers:Vec<LayerData> = Vec::new();
    let apc = decompress_apc("../resources/data.apc").unwrap();
    let start = Instant::now();

    {

    let result_layers: Vec<LayerData> = compressed_original_bytes
    .par_chunks(15)
    .enumerate()
    .map(|(index, chunk)| primitivize_layer(chunk, &apc, (index+1) as u64,layer_amount)) // Pass index to primitivize_layer
    .collect();
current_layers = result_layers;
}
    drop(compressed_original_bytes);
    layer_amount +=1;
    let mut condition_to_do = current_layers.len() !=1;
    current_layers.sort_by_key(|layer_data: &LayerData| layer_data.location);

    while condition_to_do {
        
        // Create a new vector with sorted data
        // let  uncompressed_merged_layer_data: Vec<u8> = current_layers.iter().flat_map(|layer_data| layer_data.data.clone()).collect();
        // let merged_layer_data = compress_layer_data(uncompressed_merged_layer_data);

        let  merged_layer_data: Vec<u8> = current_layers.iter().flat_map(|layer_data| layer_data.data.clone()).collect();
        current_layers.clear();
        println!("{} <-- current lenght",merged_layer_data.len());
        let result_layers: Vec<LayerData> = merged_layer_data
            .par_chunks(15)
            .enumerate()
            .map(|(index, chunk)| primitivize_layer(chunk, &apc, (index+1) as u64,layer_amount)) // Pass index to primitivize_layer
            .collect();
        current_layers = result_layers;
        condition_to_do = current_layers.len() !=1;
        layer_amount+=1;
        
    }
    let mut final_data =  current_layers[0].data.clone().to_vec();
    final_data.extend(&universal_validator);
    final_data.extend(&layer_amount.to_be_bytes());
    
    let elapsed = start.elapsed();

    println!("Elapsed time: {:?}", elapsed);
    println!("{:?} <-- final",final_data.len());
    println!("{:?} <-- final",final_data);
    
    
    let last_slash_index = file_path.rfind('/').unwrap(); 
        // Create a new file path by replacing the last element with "output.bin"
    let new_path = format!("{}{}", &file_path[..last_slash_index + 1], "example_output.alpr",);
        
        // Print the modified file path
    println!("Modified file path: {}", new_path);
    
    let mut file: File = File::create(new_path).unwrap();
    // Write the final_data into the file
    file.write_all(&final_data).unwrap();

    
}






#[tauri::command]
fn revert_from_alpr(file_path: &str) {
    println!("{:?}",file_path);
    let apc = decompress_apc("../resources/data.apc").unwrap();
    let start = Instant::now();
    // Read the file into starting_bytes
    let starting_bytes: Vec<u8> = fs::read(file_path).unwrap();
    // Extract layer_amount from the last 4 bytes
    let layer_amount_bytes = &starting_bytes[starting_bytes.len() - 4..];
    let mut layer_amount = u32::from_be_bytes([layer_amount_bytes[0], layer_amount_bytes[1], layer_amount_bytes[2], layer_amount_bytes[3]]);

    // Extract universal_validator from the last 32 bytes after layer_amount's bytes
    let universal_validator_bytes = &starting_bytes[starting_bytes.len() - 36..starting_bytes.len() - 4];

    // Create a new Vec<u8> containing the remaining bytes (excluding layer_amount and universal_validator)
    let mut primitivized_data: Vec<u8> = starting_bytes[..starting_bytes.len() - 36].to_vec();

    // Printing the extracted values and remaining_bytes for demonstration
    let mut condition_to_do = layer_amount !=0;
    

    while condition_to_do{
        let leftover_len = primitivized_data.len() % 14;
        let mut leftover = primitivized_data.split_off(primitivized_data.len() - leftover_len);

        let upper_layer_data: Vec<LayerData> = primitivized_data.par_chunks(14).enumerate()
        .map(|(index, chunk)| {
            revert_layer(chunk,&apc,(index+1) as u64,layer_amount)
        })
        .collect();
        primitivized_data.clear();
        primitivized_data = upper_layer_data.iter().flat_map(|layer_data| layer_data.data.clone()).collect();
        primitivized_data.extend(leftover);
        layer_amount -=1;
        condition_to_do = layer_amount !=0;


    }

    println!("{:?} <-- fiinal primitivized",primitivized_data);



       //write file section
    let last_slash_index = file_path.rfind('/').unwrap(); 
        // Create a new file path by replacing the last element with "output.bin"
    let new_path = format!("{}{}", &file_path[..last_slash_index + 1], "newOutput.txt",);
    if let Ok(utf8_string) = String::from_utf8(primitivized_data.clone()) {
        // Specify the file path where you want to write the UTF-8 text
       

        // Open the file for writing (this will overwrite any existing content)
        let mut file = File::create(new_path).expect("Failed to create the file");

        // Write the UTF-8 string to the file
        if let Err(err) = file.write_all(utf8_string.as_bytes()) {
            eprintln!("Error writing to file: {}", err);
        } else {
            println!("Data successfully written to");
        }
    } else {
        println!("The data is not valid UTF-8 text and cannot be written as such.");
    }

}
