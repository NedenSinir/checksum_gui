
    use std::fs;
    use std::{collections::HashMap, fs::File};
    use std::io::Write;
    use primal::is_prime;
    use serde::de::Error;
    use serde_derive::Serialize;
    use serde_derive::Deserialize;
    use serde_json::Number;
    use tauri::api::path::{BaseDirectory, resolve_path};
    use tauri::{Config, PackageInfo, Env};


    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PredefinedClass {
        pub id: String,
        //pub sum: u32,
        pub amount: u32,
        //pub is_div_by_two: [bool;3],
        pub is_prime: [bool;3],
        // pub is_div_by_three: [usize;1],
        // pub prime_amount: [usize;1],
        pub two_amount: [usize;1],
        // pub is_octal_pair:[bool;3],
        // pub total_length:[usize;1],
        // pub same_amount:u8,
        pub binary_length:[usize;1],
        // pub hex_length:[usize;1],
        // pub octal_length:[usize;1],
        pub ones:[usize;3],
        //pub members:Vec<[u32;3]>   
    }

    impl PartialEq for PredefinedClass {
        fn eq(&self, other: &Self) -> bool {
            // Compare all fields except 'id' and 'amount'
            //self.sum == other.sum &&
            //self.is_div_by_two == other.is_div_by_two &&
            //self.prime_amount == other.prime_amount &&

            self.two_amount == other.two_amount &&
            self.is_prime == other.is_prime &&
            //self.is_octal_pair == other.is_octal_pair &&
            //self.total_length == other.total_length &&
            self.binary_length == other.binary_length &&
            self.ones == other.ones
            //self.hex_length == other.hex_length &&
            //self.same_amount == other.same_amount &&
            //self.octal_length == other.octal_length


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
                        let i_octal=format!("{:o}", i).parse::<u32>().unwrap();
                        let j_octal=format!("{:o}", j).parse::<u32>().unwrap();
                        let k_octal=format!("{:0}", k).parse::<u32>().unwrap();

                 
                        let mut same_amount = 0;
                        

                        let sum = i+j+k;
                        let is_div_by_two = [i%2==0,j%2==0,k%2==0];
                        let three_bool = [i%3==0,j%3==0,k%3==0];
                        let prime_bool = [is_prime(i.into()),is_prime(j.into()),is_prime(k.into())];
                        let mut is_div_by_three = [0];
                        let mut prime_amount = [0];
                        let mut two_amount = [0];
                        let is_octal_pair = [i_octal%2==0,j_octal%2==0,k_octal%2==0];
                        let total_length = [i.to_string().len() + j.to_string().len() + k.to_string().len()];
                        let is_same = [i==j,j==k,i==k];
                        let ones = [format!("{:b}", i).matches("1").count() , format!("{:b}", j).matches("1").count() ,format!("{:b}", k).matches("1").count()];
                        let binary_length = [format!("{:b}", i).len()  + format!("{:b}", j).len() + format!("{:b}", k).len()];
                        let hex_length = [format!("{:x}", i).len() + format!("{:x}", j).len() + format!("{:x}", k).len()];
                        let octal_length = [format!("{:o}", i).len() + format!("{:o}", j).len() + format!("{:o}", k).len()];

                        prime_bool.iter().for_each(|x|{
                            if *x{
                                prime_amount[0]+=1
                            }
                        });
                        is_div_by_two.iter().for_each(|x|{
                            if *x{
                                two_amount[0]+=1
                            }
                        });
                        // let biggest_num = if i >= j && i >= k {
                        //     i
                        // } else if j >= i && j >= k {
                        //     j
                        // } else {
                        //     k
                        // };
                        
                        // let smallest_num = if i <= j && i <= k {
                        //     i
                        // } else if j <= i && j <= k {
                        //     j
                        // } else {
                        //     k
                        // };
                        // let sum: u32 = biggest_num + smallest_num;
                        // let other = (i+j+k)- sum;
                        // let mut index =0;
                        // let biggest_num_section = divide_into_sections(255, 51.0, biggest_num);
                        // let smallest_num_section = divide_into_sections(255, 51.0, smallest_num);
                        // let mut numbers_degree:[u8;2] = [0,0];
                        // let mut other_degre = 0;
                        
                        // if other%2==0{
                        //     other_degre=1
                        // }
                        
                        
                        // [biggest_num,smallest_num].iter().for_each(|x|{

                        //         numbers_degree[index] = (x % 5).try_into().unwrap() ;
                                  
                            
                        //             index+=1
                        // });
                        
                        
                            let current_object = PredefinedClass {
                            id: "".to_string(),
                            // sum:sum,
                            amount: 1,
                            // is_div_by_two,
                            // is_div_by_three,
                            // total_length,
                            // same_amount,
                            binary_length,
                            // is_octal_pair,
                            // hex_length,
                            is_prime:prime_bool,
                            //octal_length,
                            ones,
                            //prime_amount,
                            two_amount,
                            // big_section:biggest_num_section,
                            // small_section:smallest_num_section,
                            // big_small_degree :numbers_degree,
                            // other_degre:other_degre,
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
                                let id = format!("{},{}",index1,index2);
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

    fn sort_and_write_results(results: &std::collections::HashMap<u32, u32>, file_path: &str) {
        let total_count: u32 = results.values().sum();
    
        let mut sorted_entries: Vec<_> = results.iter().collect();
        sorted_entries.sort_by(|a, b| b.1.cmp(&a.1));
    
        let output_lines: Vec<String> = sorted_entries
            .iter()
            .map(|(amount, &count)| {
                let percentage = (count as f32 / total_count as f32) * 100.0;
                format!("{} -> {:.2}%", amount, percentage)
            })
            .collect();
    
        fs::write(file_path, output_lines.join("\n")).expect("Failed to write file");
    }
    
    fn read_data_from_file(file_path: &str) -> HashMap<u32, u32> {
        let data = fs::read_to_string(file_path).expect("Failed to read file");
        let mut result = HashMap::new();
    
        for line in data.lines() {
            let parts: Vec<&str> = line.split(", ").collect();
            if parts.len() == 2 {
                let amount = parts[0]
                    .trim_start_matches("Amount: ")
                    .parse::<u32>()
                    .expect("Invalid amount");
    
                let count = parts[1]
                    .trim_start_matches("Count: ")
                    .parse::<u32>()
                    .expect("Invalid count");
    
                result.insert(amount, count);
            }
        }
    
        result
    }
    pub fn generate()-> std::io::Result<()>{

        let index =33;

        let combinations = generate_combinations(255);
        //println!("{:?}", combinations);
        println!("{}", combinations.len());
        let total_amount: u32 = combinations.values().map(|x| x.amount).sum();
        println!("Total sum of amounts: {}", total_amount);
        
        let out_path = format!("../classes/outputs/output_{}_v2.txt",index);

        match write_objects_to_file(&combinations, &out_path) {
            Ok(()) => println!("Successfully wrote to file"),
            Err(e) => eprintln!("Failed to write to file: {}", e),
        }
        let am_path = format!("../classes/amounts/amount_{}.txt",index);

        match count_and_write(&combinations, &am_path) {
            Ok(()) => println!("Successfully wrote to file"),
            Err(e) => eprintln!("Failed to write to file: {}", e),
        }
        let per_path = format!("../classes/percantages/percentage_{}.txt",index);
        sort_and_write_results(&read_data_from_file(&am_path),&per_path);
        return Ok(());

    }
    // println!("{}", layer_ins);
    // println!("{}",hex::e ncode("1A0c030d"))
    //3161306330333064
    //3141306330333064
    //let decompressed_data = decompress_hex_string(&compressed_hex.its).unwrap();