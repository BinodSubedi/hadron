use std::{fs::File, io::Write};

use serde_json::Value;



pub fn filter(value:&str){

    let query_splitted: Vec<&str> = value.trim().split("`").collect();

    // print!("{:#?}",query_splitted);

    let collection_name_part:Vec<&str> = query_splitted[0].trim().split(" ").collect(); 

    // println!("{}",collection_name_part[1]);

    let parsed:Result<Value,_> = serde_json::from_str(query_splitted[1]);

    // println!("{:#?}",parsed);
    let parsed_config:Result<Value,_> = serde_json::from_str(query_splitted[query_splitted.len() - 1]);
    // println!("{:#?}",parsed_config);

    match parsed {

        Ok(output)=>{

            let serialized_json = serde_json::to_string(&output).unwrap();
            println!("Serialized JSON: {}", serialized_json);

            let file_name = collection_name_part[1].to_string() + ".json";

            if let Err(_) = save_to_file(file_name.as_str(), &serialized_json){
            eprint!("Error in creating collection"); }
            else{

                println!("Successfully created collection")

            }

        },
        Err(err)=>{
            eprint!("{err}");
        }
        
    }

    match parsed_config {

        Ok(output)=>{

            let serialized_json = serde_json::to_string(&output).unwrap();
            println!("Serialized JSON: {}", serialized_json);

            let file_name = collection_name_part[1].to_string() +"_config"+ ".json";

            if let Err(_) = save_to_file(file_name.as_str(), &serialized_json){
            eprint!("Error in creating collection"); }
            else{

                println!("Successfully created config file")

            }


        },
        Err(err)=>{
            eprint!("Error occured: {}",err)
        }
        
    }

    let file_name_save:String = "/home/qubit/Documents/hadron/.data/data/".to_string()+collection_name_part[1] + ".dat";

    File::create(file_name_save).unwrap();

    return;


}


fn save_to_file(file_name: &str, data: &str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
