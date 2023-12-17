use rocket::{serde::{Serialize,Deserialize, json::Json}, form::Form}; 
use std::{fs,env,str::FromStr};
use std::collections::HashMap;
use aes::Aes128;
use serde_json::{Value};
use std::ops::Deref;
extern crate regex;
use crate::input_and_schema_compare::comparer; 
use aes::cipher::{BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::{GenericArray,typenum::U16}};




#[derive(Serialize,Deserialize,Debug)]
#[serde(crate= "rocket::serde")]
pub struct DeleteStandardInputFormat{
    
    data: Value
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
pub struct DeleteStandardResponse{
    
    status: u16,
    response: ResponseStatus,
    data: Option<Value>

}

#[derive(PartialEq)]
enum DocumentFoundState{

    NotFound,
    AlreadyLast,
    NotInLast
}







#[delete("/<collection>/<id>",format = "json",data="<body>")]
pub async fn delete_one(collection:String,id:String, body:Json<DeleteStandardInputFormat>)-> Json<DeleteStandardResponse>{

    println!("{:?}",body);
    println!("{}", id);

    //Steps are simple
    //First implement all logic in get 
    //except we need to stop at the file where we find the match and
    //stop at the same point, appending a new member data in the file where we removed the data
    //and the data comes from last member of collection and //NOTE:: we need to delete the last
    //file if the last file has no data any longer //ALSO:: If the data removed file is last do
    //NOTHING


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
        
        let res = DeleteStandardResponse{
            
            status: 404,
            response: ResponseStatus::NotFound,
            data: None
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


   let mut foundState = DocumentFoundState::NotFound; 

   let mut deletedData:Option<Value> = None;



   let mut found_file_data:Option<Vec<Value>> = None;

   let mut last_file_data:Option<Vec<Value>> = None;

   let mut found_document_num = 1;

    for i in 1..(total_number_of_files+1){
    

        if foundState == DocumentFoundState::NotInLast{


                if i != total_number_of_files {


                    continue;

                }


        }

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
        let mut removing_index:Option<usize> = None;
        
        for (index,val) in str_splitted.iter().enumerate(){



            if let Value::Object(obj) = val{


                for (k,v) in obj {
                    

                   
                    if k == "id"{
                     if let Value::String(val) = v{

                    
                         println!("{}",val);

                         println!("{}",id);
                         if val == &id{



                             removing_index = Some(index);



                         }

                     }
                    

                    }

                }


            }


        }

        if let Some(index_val) = removing_index{

            
            deletedData = Some(str_splitted[index_val].clone());
            

            str_splitted.remove(index_val);


            //here i means variable counting loop value in total number of files in outermost loop

            if i == total_number_of_files {
                

                println!("Same file here");
            
                //here we need to write the current file with string format from str_splitted

                foundState = DocumentFoundState::AlreadyLast;
                

               found_file_data = Some(str_splitted); 

               break;

            }else{


               //Here we read the file and get the last value from the final file name and the add
               //to the file where we removed the value which is str_splitted at last add if the
               //last file has no extra data delete the file


                foundState = DocumentFoundState::NotInLast;


                found_file_data = Some(str_splitted);

                
                found_document_num = i;


            }



        }else{

            if foundState == DocumentFoundState::NotInLast && i == total_number_of_files{


                last_file_data = Some(str_splitted);


        }



        }


        
    }



    //Here we got the data where we removed the thing with the given id and the last file data

    println!("Deleted_data:{:?}",deletedData);

    println!("Deleted_file_index:{}",found_document_num);

    println!("Found_data with removed data: {:?}",found_file_data);

    println!("Last file data:{:?}",last_file_data);

    return Json(DeleteStandardResponse{

        status: 200,
        response: ResponseStatus::Success,
        data:deletedData
    });

}
