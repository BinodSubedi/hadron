use rocket::{serde::{Serialize,Deserialize, json::Json}, form::Form};
use std::{fs,env,str::FromStr};
use std::io::{Read,Write};
use std::collections::HashMap;
use serde_json::{Value};
use std::fs::OpenOptions;
use std::ops::Deref;
extern crate regex;
use crate::input_and_schema_compare::comparer; 

use aes::Aes128;
use aes::cipher::{BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::{GenericArray,typenum::U16}};





#[derive(Serialize,Deserialize,Debug)]
#[serde(crate= "rocket::serde")]
pub struct PatchStandardInputFormat{
    
    data: Value
}


#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
enum ResponseStatus{
    Success,
    Failed,
    NotFound,
    BadRequest
}

#[derive(Debug,Clone,Deserialize,Serialize)]
#[serde(crate="rocket::serde")]
pub struct PatchStandardResponse{
    
    status: u16,
    response: ResponseStatus,
    data: Value


}


fn encrypt(data:Vec<Value>, key: & GenericArray<u8,U16>)-> Vec<GenericArray<u8,U16>> {

    let mut blocks_raw:Vec<u8> = Vec::new();

    for datum in data{

   

    for &byte in datum.to_string().as_bytes().iter(){


        blocks_raw.push(byte);

    }


    println!("blocks:{:?}",blocks_raw);

    println!("{}",blocks_raw.len());


    let remainder_padd_add_len = 16- (blocks_raw.len() % 16);  


    //Adding spaces as padding up to that point and now adding making space of byte for comma

    blocks_raw.extend(vec![32;remainder_padd_add_len-1]);

    blocks_raw.push(44);


    }

    let mut blocks:Vec<GenericArray<u8, U16>> = Vec::new();


    let mut counter = 0;

    loop{    

        //println!("counterrr:{}, {}",counter,contents.len());

        blocks.push( GenericArray::from_slice(&blocks_raw[counter..(counter+16)]).clone());

        //println!("{}",&counter);

        counter = counter + 16; 


        if counter == blocks_raw.len(){

            //println!("{counter}");

            break;
        }

    }                     


    // let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&contents).clone();
    //  let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&vec_key).clone();

    let cipher = Aes128::new(key);


    // cipher.decrypt_block(&mut block);

    cipher.encrypt_blocks(&mut blocks);




    blocks


}



#[patch("/<collection>/<id>",format = "json",data="<body>")]
pub async fn patch(collection:String,id:String, body:Json<PatchStandardInputFormat>)-> Json<PatchStandardResponse>{

    println!("{:?}",body);



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
        
        let res = PatchStandardResponse{
            
            status: 404,
            response: ResponseStatus::NotFound,
            data: Value::Null
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

    for &byte in key_val.iter(){
        vec_key.push(byte);
    } 

    let key = GenericArray::from_slice(&vec_key);



   let mut deletedData:Option<Value> = None;



   let mut found_file_data:Option<Vec<Value>> = None;

   let mut patching_index:Option<usize> = None;

   let mut found_document_num = 0;

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

        //Here we need to search for the same id as provided in the request and if found
        //we need to replace the data with data in the last file
        //
        
        
        for (index,val) in str_splitted.iter().enumerate(){



            if let Value::Object(obj) = val{


                for (k,v) in obj {
                    

                   
                    if k == "id"{
                     if let Value::String(val) = v{

                    
                         println!("{}",val);

                         println!("{}",id);
                         if val == &id{


                             found_file_data = Some(str_splitted.clone());

                             patching_index = Some(index);
                    
                             found_document_num = i;

                             break;


                         }

                     }
                    

                    }

                }


            }


        }


        
    }



    if found_document_num == 0 {


        return Json(PatchStandardResponse{

            status: 200,
            response: ResponseStatus::Success,
            data:Value::Null

        });


    }



    if let Some(mut data) = found_file_data{


        //Also, we need a little more sophisticated patch process
        //through loop and changind data where we are suppose to,
        //check out for username cause in user document, ambiguity can apper while processing

        //Simply checking if the requested patch id is same as the sent data id

        if body.data["id"] == id {

            
            data[patching_index.unwrap()] = body.data.clone();

        }else{


        return Json(PatchStandardResponse{
            status: 200,
            response: ResponseStatus::BadRequest,
            data:Value::Null

    });





        }

        //Here we write the found_file_data file with newly chnaged data

        //encrypt and save the respective file data

        let encrypted_data = encrypt(data.clone(), key);

        let found_doc_name;

        let directory = String::from("/home/qubit/Documents/hadron/.data/data/");

        if found_document_num == 1 {


            found_doc_name = collection.to_lowercase() +".dat";

        }else{

        found_doc_name = collection.to_lowercase() + "-" + &(found_document_num-1).to_string() + ".dat";

        }


        println!("{}",found_doc_name);

        
        let mut found_file = OpenOptions::new().write(true).open(directory + &found_doc_name).unwrap();

        for arr in encrypted_data{

            if let Err(err) =  found_file.write_all(&arr){
                    eprintln!("{:?}",err);
                    panic!("encrypted data not written!");
            }


        }



        return Json(PatchStandardResponse{

            status: 200,
            response: ResponseStatus::Success,
            data:data[patching_index.unwrap()].clone()

        });




    }

    return Json(PatchStandardResponse{

        status: 200,
        response: ResponseStatus::Success,
        data:Value::Null

    });


}
