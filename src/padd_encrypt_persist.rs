use std::str::from_utf8;
use std::fs;
use std::io::{Read, Write};
use std::fs::{File, OpenOptions};
use aes::Aes128;
use aes::cipher::{BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::{GenericArray,typenum::U16},
};

use std::env;


pub fn padd_encrypt_persist(input_initial: Vec<String>, file_name:String){

    // It is a good idea to perform these tasks individually because
    // we also need to see the total capacity of exisiting written file, total capacity=68

    for input in input_initial{
        
        padd_and_init(input,file_name.clone());

    }
}


fn padd_and_init(raw: String,file_name:String){

    println!("raw:{}",raw);
    /*

    let strr: String = String::from("hello")+ from_utf8(&[32;3]).unwrap();


    println!("{}end",strr);

    for byte in strr.bytes(){
    

        println!("{:?}", byte);

    }

    */

    let mut blocks_raw:Vec<u8> = Vec::new();

    for &byte in raw.as_bytes().iter(){


        blocks_raw.push(byte);

    }


    println!("blocks:{:?}",blocks_raw);

    println!("{}",blocks_raw.len());
    

    let remainder_padd_add_len = 16- (blocks_raw.len() % 16);  


    //Adding spaces as padding up to that point and now adding making space of byte for comma

    blocks_raw.extend(vec![32;remainder_padd_add_len-1]);

    blocks_raw.push(44);



    println!("{}",blocks_raw.len());

    println!("{:?}",blocks_raw);

    encrypt(blocks_raw, file_name);

    //println!("value from bytes{:?}", String::from_utf8(blocks_raw).unwrap());

   
}

fn encrypt(text_val_vec:Vec<u8>, file_name:String){

 
    let env_variables: Vec<String> = env::args().collect();

    println!("{:?}",env_variables.last().unwrap());

    let key_str = env_variables.last().unwrap();

    let key_val =key_str.as_bytes();

                    if key_val.len() != 16 {
                        panic!("mismatched key");
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

                       blocks.push( GenericArray::from_slice(&text_val_vec[counter..(counter+16)]).clone());

                       //println!("{}",&counter);

                       counter = counter + 16; 


                        if counter == text_val_vec.len(){

                            //println!("{counter}");
                            
                            break;
                        }
                        
                    }                     


                   // let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&contents).clone();
                    //  let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&vec_key).clone();

                    let cipher = Aes128::new(&key);


                    // cipher.decrypt_block(&mut block);

                    cipher.encrypt_blocks(&mut blocks);

                    println!("{:?}",blocks);

                  //  cipher.decrypt_blocks(&mut blocks);

                   // println!("{:?}",blocks);

                    persist(blocks,file_name);



}


fn persist(encrypted_matrix:Vec<GenericArray<u8,U16>>, file_name:String){

  let mut file_name = file_name;

  let directory = String::from("/home/qubit/Documents/hadron/.data/data");

  //we need to check if number of elements is already complete or not
  //if not complete just append
  //if complete, create new file and append



  let current_chosen_file = fs::read(directory.clone()+"/"+&file_name+".dat").unwrap();


  println!("valuesssssss{:?}", current_chosen_file);

  if current_chosen_file.len() >1{
    

     println!("{}", current_chosen_file.len());
     
     let file_seperation = file_name.split('-').collect::<Vec<&str>>();

     println!("checking_num_parsing{:?}", file_seperation[1].parse::<u8>().unwrap());

     if current_chosen_file.len() > (68*16){
        
         if file_seperation.len() == 1{

             let _ = File::create(directory.clone()+file_seperation[1]+"-1.dat");

             file_name = file_seperation[1].to_string()+"-1";
         

         }else{

            
             let num:String = (file_seperation[1].parse::<u8>().unwrap() + 1).to_string();
             file_name = file_seperation[1].to_string()+"-"+&num;

         }
         

        

     }



  }



  let mut new_file = OpenOptions::new().append(true).open(directory+"/"+&file_name+".dat").unwrap();


                    



for arr in encrypted_matrix{

 if let Err(err) =  new_file.write_all(&arr){
                    eprintln!("{:?}",err);
                    panic!("encrypted data not written!");
                 }


    }




}





