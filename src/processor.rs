use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
//use std::ops::Deref;
//use std::rc::Rc;
//use bytes::{BytesMut, BufMut};
use std::{env, process, io, fs};
extern crate rocket;
// use rocket::response::status::NotFound;
use rocket::{Rocket, Build};
use aes::Aes128;
use aes::cipher::{BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::{GenericArray,typenum::U16},
};

use crate::routes::delete_routes::delete_one;
use crate::routes::put_routes::{put_one};
use crate::routes::post_routes::{post_one,post_user};
use crate::routes::patch_routes::patch;

use crate::routes::get_routes::{get_all, get_any_related, get_one, get_custom_filter};
use crate::input_filter_engine::query_filter;

pub fn processor() -> Result<Rocket<Build>,Box<dyn Error>>{
 
let args:Vec<String> = env::args().collect();

let directory = String::from("/home/qubit/Documents/hadron/.data/configure");

    let files = match fs::read_dir(&directory){
        Ok(val)=> {val},
      _=> {
        eprint!("{} is nowhere to be found",&directory);    
        process::exit(0)
    }  
    };

    let mut length = 0;


    for file in files {
        if let Ok(val) = file{
            // println!("{:#?}",val);

                //let inner_count = &mut len;
                length += 1;
//                println!("len:{}",inner_count);
             if val.file_type()?.is_file() {
                // Process the file
               // println!("File Name: {:?}", val.file_name());
                
                let mut opened = File::open(val.path())?;

                let mut contents = Vec::new();

                if let Ok(len) =  opened.read_to_end(&mut contents){

                   // println!("length of data {}",len);

                    if len == 0{
                    //let mut buffer = BytesMut::with_capacity(16);
                    let mut pass = String::from("");

                    println!("Please enter your password(make it 16 characters):");
                    match io::stdin().read_line(&mut pass){
                        Ok(val)=> val,
                        Err(_)=>panic!("Something unexpected in input password")
                    };

                    let mut final_str = String::from("");

                    for c in pass.chars(){
                        if c == '\r' || c == '\n' {
                            //println!("here \\r and \\n")
                            continue;
                        }else{
                           final_str.push(c);
                        }
                    };

                    //println!("{}",pass);

                    let key_val = final_str.as_bytes();

                    //buffer.put(key_val);

                    let mut vec_key = Vec::new();
                    for &byte in key_val.iter(){
                        vec_key.push(byte);
                    } 

                    //println!("{:#?}",&vec_key);

                   let key:&GenericArray<u8,U16> = GenericArray::from_slice(&vec_key);

                    // let mut block:GenericArray<u8,U16> = GenericArray::from([0u8;16]);
                    let cipher = Aes128::new(&key);

                let mut block:GenericArray<u8,U16> =  GenericArray::from_slice(&vec_key).clone();
                 
                 cipher.encrypt_block(&mut block);

                 let mut new_file = OpenOptions::new().append(true).open(directory.clone()+"/configure.dat")?;


                     if let Err(err) =  new_file.write_all(&block){
                         eprintln!("{:?}",err);
                         process::exit(0);
                     }

                     println!("Initial setup done🚀🚀");
                    process::exit(0); 

                    }
                    // println!("{:#?}",contents);

                    // let key = GenericArray::from([0u8; 16]);
                    // let key = GenericArray::from_slice();

                    let strr;
                    if args.iter().count() == 2 {
                        strr = &args[1];
                    }else if args.iter().count() == 3 {
                        strr = &args[2];
                    }else{
                        process::exit(0);
                    }



                    let key_val =strr.as_bytes();

                    if key_val.len() != 16 {
                        process::exit(0);
                    }

                    let mut vec_key = Vec::new();
                    for &byte in key_val.iter(){
                        vec_key.push(byte);
                    } 

                   let key = GenericArray::from_slice(&vec_key);

                    // let mut block:GenericArray<u8,U16> = GenericArray::from([0u8;16]);

                    let mut blocks:Vec<GenericArray<u8, U16>> = Vec::new();


                    let mut counter = 0;

                    loop{    
                    
                        //println!("counterrr:{}, {}",counter,contents.len());

                       blocks.push( GenericArray::from_slice(&contents[counter..(counter+16)]).clone());

                       //println!("{}",&counter);

                       counter = counter + 16; 


                        if counter == contents.len(){

                            //println!("{counter}");
                            
                            break;
                        }
                        
                    }                     


                   // let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&contents).clone();
                    //  let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&vec_key).clone();

                    let cipher = Aes128::new(&key);


                    // cipher.decrypt_block(&mut block);

                    cipher.decrypt_blocks(&mut blocks);

                    for block in blocks{

                        let plaintext = String::from_utf8(block.to_vec())?;

                        //println!("{:#?}",&plaintext);

                        if strr != &plaintext {
                            println!("Wrong credential entered!");
                            process::exit(0);
                        }

                    }



                    // let p_file = String::new();
                    //////////////////////////////////////
                    //write bloc

                    // let mut new_file = OpenOptions::new().append(true).open(directory.clone()+"/configure.dat")?;

                    // if let Err(err) =  new_file.write_all(&block){
                    //     eprintln!("{:?}",err);
                    //     process::exit(0);
                    // }



                }

            
            }

        };


    }


    //here below block is for creating initial setup when no file as configure.dat
    //above initial setup is when file exists but file content is empty

   // println!("len:{}",length);
    if length == 0 {

                    match File::create("/home/qubit/Documents/hadron/.data/configure/configure.dat") {
                    Ok(_)=> (),
                    Err(err)=> panic!("Failed to create file: {err}")
                    };

                    let mut pass = String::from("");

                    println!("Please enter your password(make it 16 characters):");
                    match io::stdin().read_line(&mut pass){
                        Ok(val)=> val,
                        Err(_)=>panic!("Something unexpected in input password")
                    };

                    let mut final_str = String::from("");

                    for c in pass.chars(){
                        if c == '\r' || c == '\n' {
                            println!("here \\r and \\n")
                        }else{
                           final_str.push(c);
                        }
                    };

                //    println!("{}",pass);

                    let key_val = final_str.as_bytes();



                    //let key_val = "Thats my Kung Fu".as_bytes();
                    let mut vec_key = Vec::new();
                    for &byte in key_val.iter(){
                        vec_key.push(byte);
                    } 

                    //println!("{:#?}",&vec_key);

                   let key:&GenericArray<u8,U16> = GenericArray::from_slice(&vec_key);

                    // let mut block:GenericArray<u8,U16> = GenericArray::from([0u8;16]);
                    let cipher = Aes128::new(&key);

                let mut block:GenericArray<u8,U16> =  GenericArray::from_slice(&vec_key).clone();
                 
                 cipher.encrypt_block(&mut block);

                 let mut new_file = OpenOptions::new().write(true).open(directory.clone()+"/configure.dat")?;


                     if let Err(err) =  new_file.write_all(&block){
                         eprintln!("{:?}",err);
                         process::exit(0);
                     }

                     println!("Initial setup done🚀🚀");
                    process::exit(0); 


                    //write bloc

                       }

//      dbg!(&args);
//   print!("{}",args.iter().count());

    if args.iter().count() == 2{

        println!("                                                        ..                    
        ./((###########(/                   /(###############(/            
     /(#####((////////((###(/            (####((/////////((######(*        
  /(##########################(       ,(###########################((      
*(((############################(    (###############################(/    
/((((/***/(###(((((((###########%%(  %####(///(#####((((((###########(((/   
/((((*,,,*(###(((///((###(/*/(#%%%%%%%%##(*,,,/(####(((//((###(/***/***/((/  
.((((/,,,,/####((////((###(/**(#%%%%%%%%##/,,,*(######(///((###(/****/***((/. 
/((((*,,,*(####((///(((###(/*/#%%%%%%%%%##(,,,*(######((/(((###(/****/***/((* 
,((((/**//((##(((((((((###((/(#%%%%%%%%%##(**//((####(((((((###((///((((((((. 
//(((((((((###((((((((####((##%%%%%%%%%###((((((####(((((((#####(((######(/  
/(((((((######((((((##########%%%%(  %####(((#######((((###############(/   
*/((((((((######################(    (##############################((/    
  /(((((((####################(       .(##########################((/      
     /(((###################(            (#######################(*        
        .((#############(                   *#################(   \n\n");


    println!("Hadron Query Mode::::::::::::\n");


    
    loop {
        let mut input = String::new();

    // Read user input and handle errors
    match io::stdin().read_line(&mut input) {
        Ok(_) => {

            if input == String::from("exit\r\n") || input == String::from("exit\n"){
                println!("Hadron exiting.........");
                process::exit(0);
            }else{
                // println!("{input}");
                query_filter::filter(&input);
            }
        }
        Err(_) => {
            // eprintln!("Error: {}", error);
            // println!("Hadron exiting.........");
            process::exit(0);
        }
    }
}
    }

    if args[1].to_string() == String::from("powerup") {
        Ok(rocket::build().mount("/get", routes![get_one,get_all,get_any_related,get_custom_filter]).mount("/put",routes![put_one]).mount("/post",routes![post_one,post_user]).mount("/delete",routes![delete_one]).mount("/patch", routes![patch]))     
    }

    else{
            process::exit(0);

    }

}
