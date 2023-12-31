  //1 prime hariç her şey = çok fazla
    //2 intervale ve prime hariç = 59978
    //sadece 3 ve 2 ve 5 =59978 üstteki ile aynı
    //pair 3 ve intervale = 293914
    //sadece 3 ve 2 ve 5 ve prime = 160720
    //her şey = çok fazla
    //intervalle 3 2 = 293914
    //intervalle 2 5 = 300336
    //intervalle ve 2 = 48128
    //3 2 5 ve prime  = ?

    use std::{collections::HashMap, fs::File};
    use std::io::Write;
    use serde::de::Error;
    use serde_derive::Serialize;

    //4lü intervale e pair 48128
    //compress_apcp("data.apcp","data2.apcp").unwrap();
    #[derive(Debug, Clone, Serialize)]
    struct PredefinedClass {
        id: String,
        sum_big_small: u32,
        amount: u32,
        big_section:u32,
        small_section:u32,
        same_amount:u8,
        big_small_degree:[u8;2],        
    }

    impl PartialEq for PredefinedClass {
        fn eq(&self, other: &Self) -> bool {
            // Compare all fields except 'id' and 'amount'
            self.sum_big_small == other.sum_big_small &&
            self.big_section == other.big_section &&
            self.small_section == other.small_section &&
            self.same_amount == other.same_amount &&
            self.big_small_degree == other.big_small_degree
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
    fn divide_into_sections(number: u32, step_amount: u32, target_number: u32) -> u32 {
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
                        let biggest_num_section = divide_into_sections(255, 51, biggest_num);
                        let smallest_num_section = divide_into_sections(255, 51, smallest_num);
                        let mut numbers_degree:[u8;2] = [0,0];

                        [biggest_num,smallest_num].iter().for_each(|x|{


                                    match  x % 10 {
                                        
                                        0 | 5 =>{
                                            numbers_degree[index] = 0
                                        }
                                        1 | 6=>{
                                            numbers_degree[index] = 1
                                        }
                                         2|7 => {

                                            numbers_degree[index] = 2
                                        }
                                        3|8 => {

                                            numbers_degree[index] = 3
                                        }
                                        4|9 => {

                                            numbers_degree[index] = 4
                                        }
                                        _=>{
                                            panic!("noluyo")
                                        }
                                        
                                    }

                                
                                
                            
                            
                                    index+=1
                        });
                        
                        
                            let current_object = PredefinedClass {
                                id: "".to_string(),
                                sum_big_small:sum+other,
                                amount: 1,
                   
                            big_section:biggest_num_section,
                            small_section:smallest_num_section,
                            same_amount:same_amount,
                            big_small_degree :numbers_degree,
                        };
    
        
                        let previous_object = result.values().find(|el| {**el==current_object});
                        match previous_object {
                            Some(previous_object) => {
                                let mut new_object = previous_object.clone();
                                new_object.amount += 1;
                                
                                new_object.id = previous_object.id.clone();
                                result.insert(previous_object.id.clone(), new_object);
                            }
                            None => {
                                if index1 > range{
                                    panic!("{} <-- noluyo aga",index1)
                                }
                                let id = format!("{},{}", index1, index2);
                                let mut new_object = current_object.clone();
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
        match write_objects_to_file(&combinations, "output.txt") {
            Ok(()) => println!("Successfully wrote to file"),
            Err(e) => eprintln!("Failed to write to file: {}", e),
        }
        match count_and_write(&combinations, "amounts.txt") {
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
    