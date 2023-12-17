use rocket::{serde::{Serialize,Deserialize, json::Json}, form::Form};
use std::{env, str::FromStr};
use std::fs;
use std::collections::HashMap;
use serde_json::{Value};
use std::ops::Deref;
extern crate regex;
use crate::input_and_schema_compare::comparer; 
use aes::Aes128;
use aes::cipher::{BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::{GenericArray,typenum::U16}};


#[derive(Debug,Clone,Deserialize,Serialize)]
#[serde(crate="rocket::serde")]
pub struct GetStandardInputFormat{
    
    pub data: Value,

}


#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
enum ResponseStatus{
    Success,
    Failed,
    NotFound
}

#[derive(Debug,Clone,Deserialize,Serialize)]
#[serde(crate="rocket::serde")]
pub struct GetStandardResponse{
    
    status: u16,
    response: ResponseStatus,
    data: Vec<Value>

}





#[get("/<collection>")]
pub async fn get_all(collection:String)-> Json<GetStandardResponse>{
   format!("The collection of choice for whole query is {}",collection);




    let directory = String::from("/home/qubit/Documents/hadron/.data/data");

    let files = fs::read_dir(&directory).unwrap();

    //println!("{:?}",files);

    //let models_file_vec: Vec<&str> = Vec::<&str>::new();
      
    let mut required_model_file:Option<String> = None;

    let mut total_number_of_files = 0;

    //by the way this by_ref as took the value out of the files so may want to look into this
    //sometime soon

  //  println!("{:?}", files.by_ref().count());


    for file in files{

        
        let string_file_val = file.expect("well there\'s something fishy").path();

        let string_format = string_file_val.into_os_string().into_string().expect("format change error");

        let splitted = string_format.split("/").collect::<Vec<&str>>();


        let file_name = splitted[splitted.len()-1];

       // println!("{:?}", file_name);

        let name_splitted = file_name.split(".").collect::<Vec<&str>>();

        
//        println!("{:?}",name_splitted);

 //       println!("{}", collection.to_lowercase());

        if name_splitted[0].split('-').collect::<Vec<&str>>()[0] == collection.to_lowercase().as_str(){

           required_model_file = Some(name_splitted[0].to_string());
            println!("matched");
            
            total_number_of_files +=1;

        }

            


    }

    if required_model_file == None {
        
        let res = GetStandardResponse{
            
            status: 404,
            response: ResponseStatus::NotFound,
            data: vec![Value::Null]

        };

        return Json(res);


    }


    //If got time, the file read will be made multi-threaded but for now
    //It's just single threaded



    let mut vec_key = Vec::new();

    let env_variables: Vec<String> = env::args().collect();

    println!("{:?}",env_variables.last().unwrap());

    let key_str = env_variables.last().unwrap();

    let key_val =key_str.as_bytes();

    let mut final_values_list:Vec<Value> = Vec::new();

    for &byte in key_val.iter(){
        vec_key.push(byte);
    } 

    let key = GenericArray::from_slice(&vec_key);




    for i in 1..(total_number_of_files+1){
    
        let file_name:String;

        println!("{}",i);
        if i ==1{
            

                file_name = collection.clone().to_lowercase() + ".dat";

        }else{

            file_name = collection.clone().to_lowercase() + "-" + &(i-1).to_string() + ".dat";

        }
        
        
        println!("{}",file_name);

        //here we read file and decrypt and push to some outer scope vector

//        let directory = String::from("/home/qubit/Documents/hadron/.data/data");

       let current_chosen_file = fs::read(directory.clone()+"/"+&file_name).unwrap();


        // let mut block:GenericArray<u8,U16> = GenericArray::from([0u8;16]);

        let mut blocks:Vec<GenericArray<u8, U16>> = Vec::new();


        let mut counter = 0;

        println!("total_val:{:?}", &current_chosen_file);
        println!("val:{:?}", &current_chosen_file[0..16]);

        loop{    

            //println!("counterrr:{}, {}",counter,contents.len());

            blocks.push( GenericArray::from_slice(&current_chosen_file[counter..(counter+16)]).clone());

            //println!("{}",&counter);

            counter = counter + 16; 


            if counter == current_chosen_file.len(){

                //println!("{counter}");

                break;
            }

        }                     


        // let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&contents).clone();
        //  let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&vec_key).clone();

        let cipher = Aes128::new(&key);


        // cipher.decrypt_block(&mut block);

        cipher.decrypt_blocks(&mut blocks);


        let mut final_string:String = String::from("");

        for block in blocks{


            * &mut final_string += &String::from_utf8(block.to_vec()).unwrap();


        }




        println!("{}", final_string);
        //Now the string need to split with , but only from the area where there is no {} 
        //might have to write custom splitter
        

//        let final_string_vec:Vec<&str> = final_string.split(',').collect::<Vec<&str>>();


        // Custom comma separator////////////////////
        //
            
        let mut str_splitted:Vec<Value> = Vec::new();
        
        let mut now_val:String = "".to_string();

        let mut curly_level = 0;

        for char in final_string.chars(){
            


            match char{
                
                '{' => {
                        
                    now_val.push(char);
                    curly_level +=1;


                }

                
                '}' => {
                    
                    now_val.push(char);
                    curly_level -=1;

                }

                ',' => {
                    
                    if curly_level ==1{

                    now_val.push(char);
    
                    }else{
                    
                    let pushed = now_val.clone();

                    str_splitted.push(Value::from_str(&pushed.trim()).unwrap());

                    * & mut now_val = "".to_string();


                    }

                }


                _=>{

                
                    now_val.push(char);

                }


            }



        }
        
//        println!("{:?}", str_splitted);


//         println!("{:?}",final_string_vec);       
//       println!("{:?}",final_string_vec[0]);       

        final_values_list.extend(str_splitted);

    }



//    println!("final values:{:?}",final_values_list);






   Json(GetStandardResponse{
       status:200,
       response: ResponseStatus::Success,
       data: final_values_list

   })

}

#[get("/<collection>/many/<number>")]
pub async fn get_many(collection:String,number:i8) -> String {
   format!("The query in {}, and number of element is {}",collection,number) 
}

#[get("/<collection>/one/<id>")]
pub async fn get_one(collection:String,id:i32)-> String{
   format!("The query in {collection}, and the id is {id}")
}
#[get("/<collection>/custom_filter")]
pub async fn get_custom_filter(collection:String)-> String{
   format!("The custom filter is ran in {collection} with filter custom filter")
}
