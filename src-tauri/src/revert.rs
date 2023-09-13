use std::collections::HashMap;
use std::time::Instant;

use crate::{generate_enum_combinatorics::PredefinedClass, primitivize::LayerData};
use itertools::Itertools;
use primal::is_prime;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::IntoParallelIterator;
use rayon::{prelude::ParallelIterator, slice::ParallelSlice};
use std::iter::FlatMap;
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

fn find_apc_possibilites(
    class_id: &[u8],
    apc_hashmap: &HashMap<[u8; 2], PredefinedClass>,
) -> Vec<(u8, u8, u8)> {
    let selected_class = apc_hashmap.get(class_id).unwrap();
    let binary_length_class = selected_class.binary_length[0];
    let div_by_two = selected_class.is_div_by_two;
    println!("{:?}", selected_class);
    let mut final_vec = Vec::<(u8, u8, u8)>::new();
    for i in 0..=255 {
        let mut one_amount = 0;
        let mut binary_length = 0;
        let binary_i = format!("{:b}", i);
        binary_length += binary_i.len();
        if binary_length > binary_length_class {
            continue;
        }

        if (i % 2 == 0) != div_by_two[0] {
            continue;
        }

        for j in 0..=255 {
            let binary_j = format!("{:b}", j);
            let mut one_amount_j = one_amount; // Reset two_amount for j
            let mut binary_length_j = binary_length; // Reset binary_length for j

            binary_length += binary_j.len();
            if binary_length_j > binary_length_class {
                continue;
            }

            if (j % 2 == 0) != div_by_two[1] {
                continue;
            }

            for k in 0..=255 {
                let binary_k = format!("{:b}", k);
                let mut one_amount_k = one_amount_j; // Reset two_amount for k
                let mut binary_length_k = binary_length_j; // Reset binary_length for k

                binary_length += binary_k.len();
                if binary_length_k > binary_length_class {
                    continue;
                }

                if (j % 2 == 0) != div_by_two[2] {
                    continue;
                }

                final_vec.push((i as u8, j as u8, k as u8));
            }
        }
    }
    return final_vec;
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
            generate_combinations_recursive(
                possible_combination,
                0,
                &mut current_data,
                &mut local_solutions,
            );

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
            generate_combinations_recursive(
                possible_combination,
                index + 1,
                current_data,
                solutions,
            );
        }
    }
}
pub fn revert_layer(
    data_to_be_reverted: &[u8],
    apc_hashmap: &HashMap<[u8; 2], PredefinedClass>,
    curr_location: u64,
    curr_layer_number: u32,
) -> LayerData {
    if data_to_be_reverted.len() != 14 {
        return LayerData {
            location: curr_location,
            layer_number: curr_layer_number,
            data: data_to_be_reverted.to_vec(),
        };
    }
    let mut id_vec = Vec::<u8>::new();
    let (data, curr_checksum) = data_to_be_reverted.split_at(data_to_be_reverted.len() - 4);

    println!("{:?} <-- data", data);
    let possible_combinations: Vec<Vec<(u8, u8, u8)>> = data
        .par_chunks(2)
        .map(|chunk_2| return find_apc_possibilites(chunk_2, apc_hashmap))
        .collect();

    println!("{:?} <-- len", possible_combinations[0].len());
    println!("{:?} <-- len", possible_combinations[1].len());
    println!("{:?} <-- len", possible_combinations[2].len());
    println!("{:?} <-- len", possible_combinations[3].len());
    println!("{:?} <-- len", possible_combinations[4].len());

    let start = Instant::now();

    let solutions: Vec<[u8; 15]> = possible_combinations[0]
        .par_iter()
        .flat_map(|i| {
            possible_combinations[1].par_iter().flat_map(|j| {
                possible_combinations[2].par_iter().flat_map(|k| {
                    possible_combinations[3].par_iter().flat_map(|l| {
                        possible_combinations[4].par_iter().flat_map(|m| {
                            let current_data = [
                                i.0, i.1, i.2, j.0, j.1, j.2, k.0, k.1, k.2, l.0, l.1, l.2, m.0,
                                m.1, m.2,
                            ];

                            let current_checksum = crc32fast::hash(&current_data).to_be_bytes();

                            if current_checksum.to_vec() == curr_checksum {
                                return Some(current_data);
                            } else {
                                return None;
                            }
                        })
                    })
                })
            })
        })
        .collect();

    println!("{:?} <-- time taken", start.elapsed());

    println!("{:?}", solutions);

    LayerData {
        location: curr_location,
        layer_number: curr_layer_number,
        data: solutions[0].to_vec(),
    }
}
