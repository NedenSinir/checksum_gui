use std::{borrow::BorrowMut, collections::HashMap, time::Instant, vec};

use primal::is_prime;

use crate::{
    checksum::{adler8, crc8, fletcher8},
    generate_enum_combinatorics::PredefinedClass,
};
#[derive(Debug, Clone)]
pub struct LayerData {
    pub location: u64,
    pub layer_number: u32,
    pub data: Vec<u8>,
}

fn divide_into_sections(number: u32, step_amount: f32, target_number: u32) -> u32 {
    let step = (number as f32) / (step_amount as f32);
    let mut current_number = 0.0;

    for index in 1..=step_amount as u32 {
        current_number += step;
        if (target_number as f32) < current_number {
            return index;
        } else if index == step_amount as u32 {
            return index;
        }
    }

    panic!("No valid index found for target number");
}
pub fn find_apc_class(
    layer_data: &[u8],
    apc_hashmap: &HashMap<[u8; 2], PredefinedClass>,
) -> ([u8; 2], u32) {
    let layer_data_slice: [u8; 4] = layer_data.try_into().unwrap();
    let i = layer_data_slice[0];
    let j = layer_data_slice[1];
    let k = layer_data_slice[2];
    let l = layer_data_slice[3];

    let searched_value = &[i as u32, j as u32, k as u32, l as u32];
    let mut selected_id = [0, 0];
    let mut selected_amount = 0;

    apc_hashmap.values().for_each(|x| {
        if x.members.contains(searched_value) {
            let id_parts: Vec<&str> = x.id.split(',').collect();
            if id_parts.len() == 2 {
                let id_array: [u8; 2] =
                    [id_parts[0].parse().unwrap(), id_parts[1].parse().unwrap()];
                selected_id = id_array;
                selected_amount = x.amount;
            } else {
                panic!("nasil iki degil aga")
            }
        }
    });

    return (selected_id, selected_amount);
}

pub fn find_definitive_identifier(
    layer_data: &[u8],
    apc_hashmap: &HashMap<[u8; 2], PredefinedClass>,
) -> ([u8; 2], u8) {
    let target_functions = [crc8, adler8, fletcher8];
    let (apc_id, apc_amount) = find_apc_class(layer_data, apc_hashmap);

    let mut amounts = [0, 0, 0, apc_amount];
    let results = [crc8(layer_data), adler8(layer_data), fletcher8(layer_data)];
    let binary_nums: [u8; 16] = [
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102,
    ];

    target_functions
        .iter()
        .enumerate()
        .for_each(|(index, func)| {
            for i in 0..=15usize {
                for j in 0..=15usize {
                    for k in 0..=15usize {
                        for l in 0..=15usize {
                            let curr_checksum = func(&[
                                binary_nums[i],
                                binary_nums[j],
                                binary_nums[k],
                                binary_nums[l],
                            ]);
                            if curr_checksum == results[index] {
                                amounts[index] += 1
                            }
                        }
                    }
                }
            }
        });
    let mut crc8res = [0u8; 2];
    let mut adler8res = [0u8; 2];
    let mut fletcher8res = [0u8; 2];
    hex::encode_to_slice(results[0], &mut crc8res).unwrap();
    hex::encode_to_slice(results[1], &mut adler8res).unwrap();
    hex::encode_to_slice(results[2], &mut fletcher8res).unwrap();

    let final_results = [crc8res, adler8res, fletcher8res, apc_id];
    let mut min_index = 0; // Assume the first element is the smallest
    let mut min_value = amounts[0];

    for (index, &amount) in amounts.iter().enumerate() {
        if amount < min_value {
            min_value = amount;
            min_index = index;
        }
    }
    println!("{:?}",amounts);
    return (final_results[min_index], min_index as u8);
}
pub fn find_identifer_id_class(
    identifier_ids: Vec<u8>,
    identifier_classes: &HashMap<[u8; 6], String>,
) -> [u8; 3] {
    let id_str = identifier_classes.get::<[u8; 6]>(identifier_ids.as_slice().try_into().unwrap()).unwrap();

    let final_slice:Vec<u8> = id_str.split(",").map(|x|{
        x.parse().unwrap()
    }).collect();

    return final_slice.as_slice().try_into().unwrap()//inal_slice;
}

pub fn primitivize_layer(
    data_to_be_primitivized: &[u8],
    apc_hashmap: &HashMap<[u8; 2], PredefinedClass>,
    identifier_classes: &HashMap<[u8; 6], String>,
    curr_location: u64,
    curr_layer_number: u32,
) -> LayerData {
    let data = data_to_be_primitivized; //24 u8
    if data.len() != 24 {
        println!("{:?} geldim aga", data);
        return LayerData {
            location: curr_location,
            layer_number: curr_layer_number,
            data: data.to_vec(),
        };
    }
    let mut id_vec = Vec::<u8>::new();
    let mut cheksum_bytes = [0u8; 8];
    let mut identifier_method_ids = Vec::<u8>::new();

    hex::encode_to_slice(crc32fast::hash(&data).to_be_bytes(), &mut cheksum_bytes).unwrap();
    for chunk_3 in data.chunks(4) {
        //can be parallelized but no diff inspected on tests
        let (selected_id, identifier_id) = find_definitive_identifier(chunk_3, apc_hashmap);
        id_vec.extend_from_slice(&selected_id);
        identifier_method_ids.push(identifier_id);
    }
    let identifier_id_class = find_identifer_id_class(identifier_method_ids, identifier_classes);

    id_vec.extend(identifier_id_class);
    id_vec.extend(cheksum_bytes);
    LayerData {
        location: curr_location,
        layer_number: curr_layer_number,
        data: id_vec,
    }
}
