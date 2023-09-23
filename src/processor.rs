use std::{env, process, io};
extern crate rocket;
use rocket::{Rocket, Build, Error};

use crate::routes::get_routes::{get_all, get_many, get_one};
use crate::input_filter_engine::query_filter;

pub fn processor() -> Result<Rocket<Build>,Error>{

    let args:Vec<String> = env::args().collect();

    // dbg!(&args);
    // print!("{}",args.iter().count());

    if args.iter().count() == 1{

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
        Ok(rocket::build().mount("/get", routes![get_one,get_all,get_many]))
    }
    else{
            process::exit(0);

    }

}