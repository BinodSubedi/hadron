use rocket::{serde::{Serialize,Deserialize, json::Json}, form::Form};
use std::{fs,env,str::FromStr};
use serde_json::{Value};
use std::ops::Deref;
extern crate regex;
extern crate uuid;
use uuid::Uuid;
use crate::input_and_schema_compare::comparer; 
use aes::Aes128;
use aes::cipher::{BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::{GenericArray,typenum::U16}};






#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
enum ResponseStatus{
    Success,
    Failed,
    NotFound
}

#[derive(Debug,Clone,Deserialize,Serialize)]
#[serde(crate="rocket::serde")]
pub struct PostStandardResponse{
    
    status: u16,
    response: ResponseStatus


}




#[derive(Debug,Clone,Deserialize,Serialize)]
#[serde(crate="rocket::serde")]
pub struct PostStandardInputFormat{
    
    pub data: Value,

}


#[derive(Debug,Clone,Deserialize,Serialize)]
#[serde(crate="rocket::serde")]
pub struct PostUserStandardInputFormat{
    
    pub username: String,
    pub password: String

}

#[derive(Debug,Clone,Deserialize,Serialize)]
#[serde(crate="rocket::serde")]
pub struct PostUserStandardResponse<'a>{
    
    status: u16,
    response: ResponseStatus,
    jwt:Option<&'a str>


}


 fn raw_id_formatter(values :Vec<String>)-> Vec<String>{

        // now need to split by comma in the outer most layer
     
        let mut value_list = Vec::<String>::new();

        for val in values{

        let mut curly_braces_level = 0;

        let mut final_string = String::from("");



        for char in val.chars(){
            
            match char {
                    
                '{'=>{
                    
                    final_string.push(char);
                    if curly_braces_level == 0 {
                        
                        //here goes all the logic for putting id:uuid+glue($)+superposition factor
                       //and a comma at last
                    
                        let id = uuid::Uuid::new_v4();
                        let final_id_struct = String::from(r#""id":""#) + &id.to_string()+r#"","#;
                        * &mut final_string += &final_id_struct;
                        
                       


                    }

                    curly_braces_level += 1;
            
                }

                _=>{
                    
                    final_string.push(char);                    

                }




            }
            

           

        }

        value_list.push(final_string);
        
        
            
        }


        value_list
    }


/*
trait Jsonifyable {


    fn comma_formatter(&self)-> Vec<String>;

}


impl Jsonifyable for PostStandardInputFormat{
    
    fn comma_formatter(&self)-> Vec<String>{

        // now need to split by comma in the outer most layer
        // since it is post request modified version, we need to loop in the vector of strings of
        // data

        let mut returner: Vec<String> = Vec::new();

        for val in &self.data{
        
        let mut comma_level_state = 0;

        let mut curly_braces_level=0;

        let mut final_string = String::from("");

        for char in val.chars(){
            
            match char {
                    
                '{'=>{
                    
                    final_string.push(char);
                    if curly_braces_level == 0 {
                        
                        //here goes all the logic for putting id:uuid+glue($)+superposition factor
                       //and a comma at last
                    
                        let id = Uuid::new_v4();

                        let final_id_struct = String::from("id") + &id.to_string()+",";
                        * &mut final_string += &final_id_struct;
                        
                       


                    }

                    curly_braces_level += 1;
            
                }

                '['=>{
                    
                    final_string.push(char);                    

                }

                ':' | ']' | '}' | ','=>{
                    if comma_level_state == 1{ 
                    comma_level_state -= 1;
                    final_string.push('"');
                    final_string.push(char);
                    }else{

                    final_string.push(char);

                    }


                }

                _=>{
                    if comma_level_state == 0{
                        
                        final_string.push('"');
                        final_string.push(char);
                        comma_level_state += 1;

                    }else{
                        
                        final_string.push(char);

                    }



                }



            }

           

        }

            
      returner.push(final_string);
    }


        returner

    }

    


}
*/





#[post("/<collection>",format = "json",data="<body>")]
pub async fn post_one(collection:String, body:Json<PostStandardInputFormat>)-> Json<PostStandardResponse>{


    let directory = String::from("/home/qubit/Documents/hadron/.data/data");

    let files = fs::read_dir(&directory).unwrap();

    //println!("{:?}",files);

    //let models_file_vec: Vec<&str> = Vec::<&str>::new();
      
    let mut required_model_file:Option<String> = None;


    //by the way this by_ref as took the value out of the files so may want to look into this
    //sometime soon

  //  println!("{:?}", files.by_ref().count());

    let mut total_file_num = 0;

    for file in files{

        
        let string_file_val = file.expect("well there\'s something fishy").path();

        let string_format = string_file_val.into_os_string().into_string().expect("format change error");

        let splitted = string_format.split("/").collect::<Vec<&str>>();


        let file_name = splitted[splitted.len()-1];

       // println!("{:?}", file_name);

        let name_splitted = file_name.split(".").collect::<Vec<&str>>();

        
        println!("{:?}",name_splitted);

        println!("{}", collection.to_lowercase());

        

        if name_splitted[0].split('-').collect::<Vec<&str>>()[0] == collection.to_lowercase().as_str(){

           total_file_num += 1;

           required_model_file = Some(name_splitted[0].to_string());
            println!("matched");

            //Didn't break here cause we need to find exactly the number of files where the data
            //are stored and use our magic number(super-position factor) to have a more probable
            //pinpoint to the data
        }

            


    }

    if required_model_file == None {
        
        let res = PostStandardResponse{
            
            status: 404,
            response: ResponseStatus::NotFound

        };

        return Json(res);


    }


    //Now the file or model exists so, we are going to go through files
    //searching for initially established config file to specific models
   
//    let mut established_model_schema: Json;

    println!("total_number_of_files{:?}",total_file_num);

    const config_file_location: &str = "./";

    let dirRead = fs::read_dir(config_file_location).expect("diretory read failed");
    println!("something");
    for file in dirRead{
            
     //   println!("{:?}",file.expect("error reading file").path());

        let mut config_file_path = file.expect("error while reading filr").path();

        let config_file_arr = config_file_path.to_str().expect("error while converting to str").split("/").collect::<Vec<&str>>();

      //  println!("{:?}",config_file_arr.last());

        let config_file_exact = config_file_arr.last().unwrap().to_string();
        
        // Well I kinda forgot that we also have a normal schema which is in the non-config file
        // so we could have equated to that too, but anyways we will read botht the schema file and
        // the config file

        if (collection.to_lowercase().clone()+"_config.json") == config_file_exact{
    
            println!("Found the config file!!");

            println!("{}",&config_file_exact);
//            println!("{}",format!("./{}.json",&collection.to_lowercase()));

            let readFileConf = String::from_utf8(fs::read(format!("./{}", &config_file_exact)).unwrap()).unwrap();
            let readFileSchema = String::from_utf8(fs::read(format!("./{}.json", &collection.to_lowercase())).unwrap()).unwrap();

            
    //         println!("{:?}", readFileConf);
 //           println!("{:?}", readFileSchema);
            
            let readFileConf_jsonified: Value = serde_json::from_str(&readFileConf).unwrap();

            

 //           println!("{:?}", readFileConf_jsonified.changeMe());
           
            let readFileSchema_jsonified: Value = serde_json::from_str(&readFileSchema).unwrap();

/*

            if let Value::Object(obj) = cheking_context{
                
                for (k,v) in obj.iter() {
                    
                    println!("If this works I will look very stupid:{},{}",k,v);

                }

            }

*/
 //           println!("{:?}", readFileSchema_jsonified.changeMe());
//            println!("{:?}", readFileSchema_jsonified["age"]["type"]);
//


            // 1.)Now below this section is supposed to be input value field and type checking
            // 2.)Encrypting by applying padding (total bytes%16, remainder + (16-remainder) =>
            //   which will be the padding)
            // 3.) Decrypting back jsut to check if worked properly

                
        //    println!("{:?}", body);
        //
        //
                
            


            //let value_input = body.deref().comma_formatter();



            let mut input_value_raw_formatted : Vec<String>  = Vec::new();

            if let Value::Array(ref value_input) =&body.data{


            for val in value_input{


//`          let itterable_body_data:Value = serde_json::from_str(&val).unwrap();               
             let raw_value_unit = Value::to_string(val);

             input_value_raw_formatted.push(raw_value_unit);


            }


            }
            

            println!("{:?}", input_value_raw_formatted);

            let raw_with_id=raw_id_formatter(input_value_raw_formatted);


           let mut file_name = collection.clone().to_lowercase();

            if total_file_num >1{
                
                let adder = format!("-{}",total_file_num-1);

               * &mut file_name += &adder; 


            } 



           // let itterable_body_data = &body.deref();
 
           

            
            comparer::schema_comparer_many(body.data.clone(),readFileSchema_jsonified,raw_with_id,file_name,total_file_num); 
            


        }
        


    }





    return Json(PostStandardResponse{ 
        status : 200,
        response : ResponseStatus::Success 

    })

}

#[post("/user/<collection>",format = "json",data="<body>")]
pub async fn post_user<'a>(collection:String, body:Json<PostUserStandardInputFormat>)-> Json<PostUserStandardResponse<'a>>{
   
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
        
        let res = PostUserStandardResponse{
            
            status: 404,
            response: ResponseStatus::NotFound,
            jwt: None
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

                let mut username_matched = false;

                let mut password_matched = false;

                for (k,v) in obj {
                    

                   
                    if k == "username"{
                     if let Value::String(username) = v{

                    
                         println!("{}",val);

                         if username == &body.username{


                             println!("Username matched!");
                             //found_file_data = Some(str_splitted.clone());

                             //patching_index = Some(index);
                    
                             //found_document_num = i;
                             username_matched = true;

                             if password_matched {
                                 println!("Both matched!");
                                 found_file_data = Some(str_splitted.clone());

                                 patching_index = Some(index);

                                 found_document_num = i;

                                 break;


                             }


                         }

                     }
                    

                    }else if k == "password"{

                     if let Value::String(password) = v{

                    
                         if password == &body.password{

                             println!("Password matched!");

                             password_matched = true;

                             if username_matched {
                             println!("Both matched!");
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


        
    }



    if found_document_num == 0 {


        return Json(PostUserStandardResponse{

            status: 404,
            response: ResponseStatus::NotFound,
            jwt:None

        });


    }

    if let Some(data) = found_file_data{

        println!("{:?}",[patching_index.unwrap()].clone());


        return Json(PostUserStandardResponse{

            status: 200,
            response: ResponseStatus::Success,
            jwt:Some("we will finish this")
        });
    }else{

        Json(PostUserStandardResponse{ status: 400, response: ResponseStatus::Failed,jwt:None})


    }






}
















