use std::collections::HashMap;
use std::time::Instant;

use itertools::Itertools;
use rayon::{slice::ParallelSlice, prelude::ParallelIterator};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use crate::{primitivize::LayerData, generate_classes::PredefinedClass};
fn generate_permutations(tuples: Vec<(u32, u32, u32)>) -> Vec<(u8, u8, u8)> {
    let mut result = Vec::new();

    for tuple in tuples {
        let v = vec![tuple.0, tuple.1, tuple.2];
        let permutations = v.into_iter().permutations(3).unique().map(|perm| {
            let tuple = (perm[0] as u8, perm[1] as u8, perm[2] as u8);
            tuple
        });

        for perm in permutations {
            result.push(perm);
        }
    }

    result
}


fn find_apc_possibilites(class_id:&[u8],apc_hashmap:&HashMap<[u8;2],PredefinedClass>)->Vec<(u8,u8,u8)>{

    let selected_class = apc_hashmap.get(class_id).unwrap();

    let big_number = selected_class.big_section*5 - (5 - selected_class.big_small_degree[0] as u32);
    let small_number = selected_class.small_section*5 - (5- selected_class.big_small_degree[1] as u32);
    let is_other_pair =  selected_class.other_degre == 1;


    // Generate all valid integer values for other_number
    let mut possible_other_numbers = Vec::new();
    for num in small_number..=big_number {
        if is_other_pair && num % 2 == 0 {
            possible_other_numbers.push(num);
        } else if !is_other_pair && num%2 !=0  {
            possible_other_numbers.push(num);
        }
    };
    let final_vec:Vec<(u32,u32,u32)> = possible_other_numbers.iter().map(|x|{
        (big_number,x.clone(),small_number)
    }).collect();
    println!("{:?} <-- combinations",final_vec.len());
    generate_permutations(final_vec)
    

}


fn find_combinations(
    possible_combinations: &Vec<Vec<(u8, u8, u8)>>,
    main_checksum: &[u8],
) -> Vec<[u8; 15]> {
    // Create a parallel iterator over the possible_combinations
    let solutions: Vec<[u8; 15]> = possible_combinations
        .par_iter()
        .flat_map(|possible_combination| {
            // For each possible combination, generate and check combinations in parallel
            let mut local_solutions = Vec::new();
            let mut current_data = [0; 15];

            // Use your existing recursive function or logic here
            generate_combinations_recursive(possible_combination, 0, &mut current_data, &mut local_solutions);

            local_solutions
        })
        .filter(|solution| {
            // Filter valid solutions based on checksum
            let current_checksum = crc32fast::hash(solution).to_be_bytes();
            current_checksum.to_vec() == main_checksum
        })
        .collect();

    solutions
}

fn generate_combinations_recursive(
    possible_combination: &[(u8, u8, u8)],
    index: usize,
    current_data: &mut [u8; 15],
    solutions: &mut Vec<[u8; 15]>,
) {
    if index == current_data.len() {
        // Base case: All elements in current_data have been filled
        solutions.push(*current_data);
    } else {
        // Recursive case: Fill the current_data with combinations
        for (a, b, c) in possible_combination {
            current_data[index * 3] = *a;
            current_data[index * 3 + 1] = *b;
            current_data[index * 3 + 2] = *c;
            generate_combinations_recursive(possible_combination, index + 1, current_data, solutions);
        }
    }
}
pub fn revert_layer(data_to_be_reverted:&[u8],apc_hashmap:&HashMap<[u8;2],PredefinedClass>,curr_location:u64,curr_layer_number:u32)->LayerData{

    if data_to_be_reverted.len() != 14 {
       return  LayerData { location: curr_location, layer_number: curr_layer_number, data: data_to_be_reverted.to_vec() }
    }
    let mut id_vec = Vec::<u8>::new();
    let (data, curr_checksum) = data_to_be_reverted.split_at(data_to_be_reverted.len() - 4);
    
    println!("{:?} <-- data",data);
    let possible_combinations:Vec<Vec<(u8,u8,u8)>> = data.par_chunks(2).map(|chunk_2|{

        return find_apc_possibilites(chunk_2, apc_hashmap)

    }).collect();

    println!("{:?} <-- len",possible_combinations[0].len());
    println!("{:?} <-- len",possible_combinations[1].len());
    println!("{:?} <-- len",possible_combinations[2].len());
    println!("{:?} <-- len",possible_combinations[3].len());
    println!("{:?} <-- len",possible_combinations[4].len());

    let mut index = 0;
    let mut solutions = Vec::new();
    let start = Instant::now();

    
    for i in possible_combinations[0].iter() {
        for j in possible_combinations[1].iter() {
            for k in possible_combinations[2].iter() {
                for l in possible_combinations[3].iter() {
                    for m in possible_combinations[4].iter() {
                        



                        let current_data = &[
                            i.0 as u8, i.1 as u8, i.2 as u8, j.0 as u8, j.1 as u8, j.2 as u8,
                            k.0 as u8, k.1 as u8, k.2 as u8, l.0 as u8,l.1 as u8, l.2 as u8, m.0 as u8,m.1 as u8, m.2 as u8
                        ];
                        index+=1;
                        let current_checksum = crc32fast::hash(current_data).to_be_bytes();
                        if current_checksum.to_vec() == curr_checksum {

                            solutions.push(*current_data);
                            
                        }
                    }
                }
            }
        }
    }
    
    println!("{:?} <-- time taken",start.elapsed());
    // At this point, 'solutions' contains the valid combinations found in parallel
    for solution in solutions.iter() {
        println!("{:?},<-- found", solution);
    }


    LayerData { location: curr_location, layer_number: curr_layer_number, data: solutions[0].to_vec() }



}



