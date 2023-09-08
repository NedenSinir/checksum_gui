
    use std::{collections::HashMap, fs::File};
    use std::io::Write;
    use serde::de::Error;
    use serde_derive::Serialize;
    use serde_derive::Deserialize;
    use tauri::api::path::{BaseDirectory, resolve_path};
    use tauri::{Config, PackageInfo, Env};


    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PredefinedClass {
        pub id: String,
        pub sum_big_small: u32,
        pub amount: u32,
        pub big_section:u32,
        pub small_section:u32,
        pub big_small_degree:[u8;2],
        pub other_degre:u32 ,
        //pub members:Vec<[u32;3]>   
    }

    impl PartialEq for PredefinedClass {
        fn eq(&self, other: &Self) -> bool {
            // Compare all fields except 'id' and 'amount'
            self.sum_big_small == other.sum_big_small &&
            self.big_section == other.big_section &&
            self.small_section == other.small_section &&
            self.big_small_degree == other.big_small_degree &&
            self.other_degre == other.other_degre
            
        }
    }
    impl Eq for PredefinedClass {}

    fn write_objects_to_file(objects: &HashMap<String, PredefinedClass>, file_path: &str) -> std::io::Result<()> {

        let mut file = File::create(file_path)?;
        for object in objects.values() {
            let json = serde_json::to_string(object).unwrap();
            writeln!(file, "{}", json)?;
        }
        Ok(())
    }
    fn count_and_write(objects: &HashMap<String, PredefinedClass>, file_path: &str) -> std::io::Result<()> {
        let mut counts: HashMap<u32, usize> = HashMap::new();
        for object in objects.values() {
            *counts.entry(object.amount).or_insert(0) += 1;
        }
    
        let mut file = File::create(file_path)?;
        for (amount, count) in &counts {
            writeln!(file, "Amount: {}, Count: {}", amount, count)?;
        }
        Ok(())
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
    
    fn generate_combinations(range: u32) -> HashMap<String, PredefinedClass> {
        let mut result: HashMap<String, PredefinedClass> = HashMap::new();
        let mut index1:u32 = 0;
        let mut index2:u32 = 0;
        
    
        for i in 0..=range {
            for j in 0..=range {
                for k in 0..=range {
                        

                 
                        let mut same_amount = 0;
       
                     
                        let is_same = [i==j,j==k,i==k];
                                               
                        is_same.iter().for_each(|x|{
                            if *x{
                                same_amount+=1
                            }
                        });
                        let biggest_num = if i >= j && i >= k {
                            i
                        } else if j >= i && j >= k {
                            j
                        } else {
                            k
                        };
                        
                        let smallest_num = if i <= j && i <= k {
                            i
                        } else if j <= i && j <= k {
                            j
                        } else {
                            k
                        };
                        let sum: u32 = biggest_num + smallest_num;
                        let other = (i+j+k)- sum;
                        let mut index =0;
                        let biggest_num_section = divide_into_sections(255, 51.0, biggest_num);
                        let smallest_num_section = divide_into_sections(255, 51.0, smallest_num);
                        let mut numbers_degree:[u8;2] = [0,0];
                        let mut other_degre = 0;
                        
                        if other%2==0{
                            other_degre=1
                        }
                        
                        
                        [biggest_num,smallest_num].iter().for_each(|x|{

                                numbers_degree[index] = (x % 5).try_into().unwrap() ;
                                  
                            
                                    index+=1
                        });
                        
                        
                            let current_object = PredefinedClass {
                            id: "".to_string(),
                            sum_big_small:sum,
                            amount: 1,
                            big_section:biggest_num_section,
                            small_section:smallest_num_section,
                            big_small_degree :numbers_degree,
                            other_degre:other_degre,
                            //members:[].to_vec()
                        };
    
        
                        let previous_object = result.values().find(|el| {**el==current_object});
                        match previous_object {
                            Some(previous_object) => {
                                let mut new_object = previous_object.clone();
                                new_object.amount += 1;
                                //new_object.members.push([i,j,k]);
                                
                                new_object.id = previous_object.id.clone();
                                result.insert(previous_object.id.clone(), new_object);

                            }
                            None => {
                                if index1 > range{
                                    panic!("{} <-- noluyo aga",index1)
                                }
                                let id = format!("{},{}", index1, index2);
                                let mut new_object = current_object.clone();
                                
                                //new_object.members.push([i,j,k]);
                                new_object.id = id.clone();
                               
                                match result.insert(id.clone(), new_object) {
                                    Some(previous_value)=>{
                                        panic!("replaced something")
                                    }
                                    None=>{}
                                    
                                };
                               
                                if index2 == range {
                                    index2 = 0;
                                    index1 += 1;
                                    
                                } else {
                                    
                                    index2 += 1;
                                }
                    }
                        }
                    }
                
            }
            println!("{:?}",result.values().len());
            println!("{:?}",i);
        }
    
        result
    }
    pub fn generate()-> std::io::Result<()>{

        

        let combinations = generate_combinations(255);
        //println!("{:?}", combinations);
        println!("{}", combinations.len());
        let total_amount: u32 = combinations.values().map(|x| x.amount).sum();
        println!("Total sum of amounts: {}", total_amount);
        

        match write_objects_to_file(&combinations, "../resources/output.txt") {
            Ok(()) => println!("Successfully wrote to file"),
            Err(e) => eprintln!("Failed to write to file: {}", e),
        }
        match count_and_write(&combinations, "../resources/amounts.txt") {
            Ok(()) => println!("Successfully wrote to file"),
            Err(e) => eprintln!("Failed to write to file: {}", e),
        }

        return Ok(());

    }
    // println!("{}", layer_ins);
    // println!("{}",hex::e ncode("1A0c030d"))
    //3161306330333064
    //3141306330333064
    //let decompressed_data = decompress_hex_string(&compressed_hex.its).unwrap();