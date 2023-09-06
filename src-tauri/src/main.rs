// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::{File, self}, vec, time::Instant};

use app::{compress::{compress_apc, decompress_apc, compress_layer_data}, generate_classes::generate, primitivize::{primitivize_layer, LayerData}};
use tauri::{Env, Context, Assets};
use ring::digest;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_apc,
            compress_to_apc,
            decompress_from_apc,
            primitivize_to_alpr
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
fn primitivize_to_alpr(file_path: &str) {
    

    // Open the file for reading
    
    // Read the contents of the file into a Vec<u8>
    let original_bytes = fs::read_to_string(file_path).unwrap().as_bytes().to_vec();
    let compressed_original_bytes = compress_layer_data(original_bytes);
    let universal_validator:Vec<u8> = digest::digest(&digest::SHA256, &compressed_original_bytes).as_ref().to_vec();
    let mut layer_amount = 1;
    let mut current_layers:Vec<LayerData> = vec![];
    let apc = decompress_apc("../resources/data.apc").unwrap();
    {
    let mut location = 1;

    for chunk in compressed_original_bytes.chunks(15) {
        if chunk.len() != 15{
            continue;
        } 

        let  primitivized_layer = primitivize_layer(chunk,&apc,location,layer_amount);
        current_layers.push(primitivized_layer);
        location +=1;

    }
    }
    drop(compressed_original_bytes);
    layer_amount +=1;
    let mut condition_to_do = current_layers.len() !=1;
    current_layers.sort_by_key(|layer_data: &LayerData| layer_data.location);
    let start = Instant::now();

    while condition_to_do {
        
        // Create a new vector with sorted data
        // let  uncompressed_merged_layer_data: Vec<u8> = current_layers.iter().flat_map(|layer_data| layer_data.data.clone()).collect();
        // let merged_layer_data = compress_layer_data(uncompressed_merged_layer_data);

        let  merged_layer_data: Vec<u8> = current_layers.iter().flat_map(|layer_data| layer_data.data.clone()).collect();
        current_layers.clear();
        
        let mut location = 1;
        println!("{} <-- current lenght",merged_layer_data.len());
        for chunk in merged_layer_data.chunks(15) {
            //* */
            
            if chunk.len() != 15{
                continue;
            } 
            let  primitivized_layer = primitivize_layer(chunk,&apc,location,layer_amount);//11.4ms aşşa yukarı
            
            
            location +=1;
            current_layers.push(primitivized_layer);
            
            //* */
        }
        condition_to_do = current_layers.len() !=1;
        layer_amount+=1;
        
    }
    let mut final_data =  current_layers[0].data.clone().to_vec();
    final_data.extend(&universal_validator);
    final_data.extend(&layer_amount.to_be_bytes());
    
    let elapsed = start.elapsed();

    println!("Elapsed time: {:?}", elapsed);
    println!("{:?} <-- final",final_data);
    
    
}



