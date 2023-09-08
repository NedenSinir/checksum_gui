use std::{collections::HashMap, borrow::BorrowMut, vec, time::Instant};

use primal::is_prime;

use crate::generate_classes_new::PredefinedClass;
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
        }
        else if  index == step_amount  as u32{
            return  index;
        } 
            
        
    }
    
    panic!("No valid index found for target number");
}
pub fn find_apc_class(layer_data:&[u8],apc_hashmap:&HashMap<[u8;2],PredefinedClass>)->[u8;2]{
    let layer_data_slice:[u8;3] = layer_data.try_into().unwrap();
    let i = &layer_data_slice[0];
    let j = &layer_data_slice[1];
    let k = &layer_data_slice[2];
    let is_div_by_two = [i%2==0,j%2==0,k%2==0];
    let prime_bool = [is_prime((*i).into()),is_prime((*j).into()),is_prime((*k).into())];
    let mut prime_amount = [0];
    let mut two_amount = [0];
    let ones = [format!("{:b}", i).matches("1").count() , format!("{:b}", j).matches("1").count() ,format!("{:b}", k).matches("1").count()];
    let binary_length = [format!("{:b}", i).len()  + format!("{:b}", j).len() + format!("{:b}", k).len()];
    is_div_by_two.iter().for_each(|x|{
        if *x{
            two_amount[0]+=1
        }
    });
    let current_object = PredefinedClass {
        id: "".to_string(), //doesn't count for partial eq
        amount: 1,//doesn't count for partial eq
        binary_length,
        ones,
        is_prime:prime_bool,
        two_amount
        //members:[].to_vec()//doesn't count for partial eq
    };

    
    let selected_class: &PredefinedClass = apc_hashmap.values().find(|&x| *x == current_object).unwrap();


    

    let values: Vec<&str> = selected_class.id.split(',').collect();
    let selected_id: [u8; 2] = [
        values[0].parse().unwrap(),
        values[1].parse().unwrap(),
    ];
    return selected_id;
}






pub fn primitivize_layer(data_to_be_primitivized:&[u8],apc_hashmap:&HashMap<[u8;2],PredefinedClass>,curr_location:u64,curr_layer_number:u32)->LayerData{
    
    let data = data_to_be_primitivized;//15 u8
    if data.len() != 15 {
       return  LayerData { location: curr_location, layer_number: curr_layer_number, data: data.to_vec() }
    }
    let mut id_vec = Vec::<u8>::new();
    let main_checksum = crc32fast::hash(&data).to_be_bytes().to_vec();
    
    for chunk_3 in data.chunks(3){ //can be parallelized but no diff inspected on tests
        let current_class_id = find_apc_class(chunk_3, apc_hashmap);
        
        
        id_vec.extend_from_slice(&current_class_id);
    
    }
    id_vec.extend(main_checksum);
    LayerData { location: curr_location, layer_number: curr_layer_number, data: id_vec }

}